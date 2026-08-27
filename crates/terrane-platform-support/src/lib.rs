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
use std::io::{Read, Write as _};
use std::net::{IpAddr, Shutdown, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
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

#[derive(Clone)]
pub struct Capability(Arc<CapabilityInner>);

enum CapabilityInner {
    Secure,
    Invalid(String),
    Pseudo(Mutex<rand_chacha::ChaCha20Rng>),
    Secret(Mutex<Zeroizing<Vec<u8>>>),
    Cancellation(AtomicBool),
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
    Capability(Arc::new(CapabilityInner::Secret(Mutex::new(
        Zeroizing::new(data),
    ))))
}

pub fn destroy_secret(secret: &Capability) -> ResultValue {
    let CapabilityInner::Secret(secret) = secret.0.as_ref() else {
        return ResultValue::error("capability is not a secret buffer");
    };
    secret.lock().expect("secret lock poisoned").clear();
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
    if secret.is_empty() {
        return ResultValue::error("secret buffer was destroyed");
    }
    let output = match algorithm {
        "sha-256" => {
            let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(secret.as_slice())
                .expect("HMAC accepts every key length");
            mac.update(data);
            mac.finalize().into_bytes().to_vec()
        }
        "sha-512" => {
            let mut mac = hmac::Hmac::<sha2::Sha512>::new_from_slice(secret.as_slice())
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
    data.iter().map(|byte| format!("{byte:02x}")).collect()
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
    nesting_limit: i128,
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
    let Ok(nesting_limit) = usize::try_from(nesting_limit) else {
        return ResultValue::limit("decompression nesting limit must be positive");
    };
    if ratio_limit <= 0 || nesting_limit == 0 {
        return ResultValue::limit("decompression ratio and nesting limits must be positive");
    }
    let mut current = input.to_vec();
    let mut current_format = format;
    let mut work = input.len();
    for depth in 0..nesting_limit {
        if work > work_limit {
            return ResultValue::limit("decompression work-byte limit exceeded");
        }
        let maximum = output_limit
            .min(work_limit.saturating_sub(work))
            .saturating_add(1);
        let data = match decompress_one(current_format, &current, maximum, output_limit) {
            Ok(value) => value,
            Err(error) => return error,
        };
        work = work.saturating_add(data.len());
        if data.len() > output_limit || work > work_limit {
            return ResultValue::limit("decompression output or work-byte limit exceeded");
        }
        if !current.is_empty() && data.len() as u128 > current.len() as u128 * ratio_limit as u128 {
            return ResultValue::limit("decompression ratio limit exceeded");
        }
        let Some(next_format) = compressed_format(&data) else {
            return ResultValue {
                data,
                ..ResultValue::default()
            };
        };
        if depth + 1 == nesting_limit {
            return ResultValue::limit("decompression nesting limit exceeded");
        }
        current = data;
        current_format = next_format;
    }
    ResultValue::limit("decompression nesting limit exceeded")
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

fn compressed_format(data: &[u8]) -> Option<&'static str> {
    if data.starts_with(&[0x1f, 0x8b]) {
        Some("gzip")
    } else if data.starts_with(&[0x28, 0xb5, 0x2f, 0xfd]) {
        Some("zstd")
    } else if data.len() >= 2 && data[0] == 0x78 && u16::from_be_bytes([data[0], data[1]]) % 31 == 0
    {
        Some("zlib")
    } else {
        None
    }
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
    if text.is_empty() || text.len() > 253 || !text.is_ascii() {
        return ResultValue::error("host name must contain 1..=253 ASCII bytes");
    }
    let valid = text.split('.').all(|label| {
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
        text: text.to_ascii_lowercase(),
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
fn timeout(milliseconds: i128) -> Result<Duration, ResultValue> {
    let value = u64::try_from(milliseconds)
        .ok()
        .filter(|value| *value > 0)
        .ok_or_else(|| ResultValue::error("deadline must be positive milliseconds"))?;
    Ok(Duration::from_millis(value))
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
        .filter_map(|entry| entry.parse::<IpAddr>().ok())
        .map(|ip| SocketAddr::new(ip, port))
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
    let (sender, receiver) = std::sync::mpsc::channel();
    for (index, address) in candidates.into_iter().enumerate() {
        let sender = sender.clone();
        std::thread::spawn(move || {
            let delay = Duration::from_millis((index as u64).saturating_mul(250));
            if delay >= remaining {
                return;
            }
            std::thread::sleep(delay);
            if let Ok(stream) =
                TcpStream::connect_timeout(&address, remaining.checked_sub(delay).unwrap())
            {
                let _ = sender.send((stream, address));
            }
        });
    }
    drop(sender);
    let poll = Duration::from_millis(25);
    loop {
        if let Some(error) = cancellation_error(cancellation) {
            return error;
        }
        let elapsed = started.elapsed();
        if elapsed >= duration {
            return ResultValue {
                failed: true,
                deadline_exceeded: true,
                message: "TCP connect deadline exceeded".to_owned(),
                ..ResultValue::default()
            };
        }
        let remaining = duration.checked_sub(elapsed).unwrap();
        match receiver.recv_timeout(poll.min(remaining)) {
            Ok((stream, peer)) => {
                return ResultValue {
                    text: peer.to_string(),
                    capability: Some(Capability(Arc::new(CapabilityInner::Tcp(Mutex::new(
                        Some(stream),
                    ))))),
                    ..ResultValue::default()
                };
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                return ResultValue::error("all TCP connection candidates failed");
            }
        }
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
    let guard = listener.lock().expect("listener lock poisoned");
    let Some(listener) = guard.as_ref() else {
        return ResultValue::error("listener is closed");
    };
    if let Err(error) = listener.set_nonblocking(true) {
        return ResultValue::error(error.to_string());
    }
    let started = std::time::Instant::now();
    loop {
        if let Some(error) = cancellation_error(cancellation) {
            let _ = listener.set_nonblocking(false);
            return error;
        }
        match listener.accept() {
            Ok((stream, peer)) => {
                let _ = listener.set_nonblocking(false);
                if let Err(error) = stream.set_read_timeout(Some(duration)) {
                    return io_error("TCP read timeout configuration", &error);
                }
                if let Err(error) = stream.set_write_timeout(Some(duration)) {
                    return io_error("TCP write timeout configuration", &error);
                }
                return ResultValue {
                    text: peer.to_string(),
                    capability: Some(Capability(Arc::new(CapabilityInner::Tcp(Mutex::new(
                        Some(stream),
                    ))))),
                    ..ResultValue::default()
                };
            }
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    && started.elapsed() < duration =>
            {
                std::thread::sleep(Duration::from_millis(10))
            }
            Err(error) => {
                let _ = listener.set_nonblocking(false);
                return io_error("TCP accept", &error);
            }
        }
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
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(value) => value,
        Err(error) => return ResultValue::error(format!("cannot start DNS resolver: {error}")),
    };
    let resolver = match hickory_resolver::Resolver::builder_tokio() {
        Ok(builder) => builder.build(),
        Err(error) => return ResultValue::error(format!("cannot configure DNS resolver: {error}")),
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
                entries: values
                    .iter()
                    .map(|ip| SocketAddr::new(ip, port).to_string())
                    .collect(),
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
    let roots = webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .cloned()
        .collect::<rustls::RootCertStore>();
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    let Ok(name) = rustls::pki_types::ServerName::try_from(server_name.to_owned()) else {
        return ResultValue::error("invalid TLS server name");
    };
    let Ok(connection) = rustls::ClientConnection::new(Arc::new(config), name) else {
        return ResultValue::error("cannot create TLS client");
    };
    let mut tls = rustls::StreamOwned::new(connection, tcp);
    match tls.flush() {
        Ok(()) => ResultValue {
            capability: Some(Capability(Arc::new(CapabilityInner::Tls(Mutex::new(
                Some(tls),
            ))))),
            ..ResultValue::default()
        },
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
    fn oversized_udp_datagram_reports_truncation() {
        let receiver = udp_bind("127.0.0.1:0");
        let sender = udp_bind("127.0.0.1:0");
        assert!(!receiver.failed && !sender.failed);
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
}
