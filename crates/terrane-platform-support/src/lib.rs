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
use std::net::{
    IpAddr, Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs as _, UdpSocket,
};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use subtle::ConstantTimeEq as _;
use zeroize::Zeroizing;

#[derive(Clone, Default)]
pub struct ResultValue {
    pub failed: bool,
    pub resource_limit: bool,
    pub truncated: bool,
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
    fn error(message: impl Into<String>) -> Self {
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
    Pseudo(Mutex<rand_chacha::ChaCha20Rng>),
    Secret(Zeroizing<Vec<u8>>),
    Listener(Mutex<Option<TcpListener>>),
    Tcp(Mutex<Option<TcpStream>>),
    Udp(Mutex<Option<UdpSocket>>),
    Tls(Mutex<Option<rustls::StreamOwned<rustls::ClientConnection, TcpStream>>>),
}

impl Default for Capability {
    fn default() -> Self {
        Self(Arc::new(CapabilityInner::Tcp(Mutex::new(None))))
    }
}

fn count(value: i128, label: &str) -> Result<usize, ResultValue> {
    usize::try_from(value).map_err(|_| {
        ResultValue::error(format!(
            "{label} must be a non-negative platform-sized integer"
        ))
    })
}

pub fn secure_random() -> Capability {
    Capability(Arc::new(CapabilityInner::Secure))
}

pub fn pseudo_random(seed: &[u8]) -> Capability {
    let mut expanded = [0_u8; 32];
    let digest = sha2::Sha256::digest(seed);
    expanded.copy_from_slice(&digest);
    Capability(Arc::new(CapabilityInner::Pseudo(Mutex::new(
        rand_chacha::ChaCha20Rng::from_seed(expanded),
    ))))
}

pub fn secret_buffer(data: Vec<u8>) -> Capability {
    Capability(Arc::new(CapabilityInner::Secret(Zeroizing::new(data))))
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
                    capability: Some(pseudo_random(&seed.data)),
                    ..ResultValue::default()
                }
            }
        }
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
    let output = match algorithm {
        "sha-256" => {
            let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(secret)
                .expect("HMAC accepts every key length");
            mac.update(data);
            mac.finalize().into_bytes().to_vec()
        }
        "sha-512" => {
            let mut mac = hmac::Hmac::<sha2::Sha512>::new_from_slice(secret)
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
    if text.len() % 2 != 0 || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return ResultValue::error("invalid strict hexadecimal encoding");
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
        Ok(value) => ResultValue {
            text: value.hyphenated().to_string(),
            data: value.as_bytes().to_vec(),
            ..ResultValue::default()
        },
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
pub fn uuid_v7() -> ResultValue {
    let value = uuid::Uuid::now_v7();
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
    if ratio_limit <= 0 || nesting_limit <= 0 {
        return ResultValue::limit("decompression ratio and nesting limits must be positive");
    }
    if input.len() > work_limit {
        return ResultValue::limit("decompression work limit exceeded");
    }
    let reader: Box<dyn Read> = match format {
        "gzip" => Box::new(flate2::read::GzDecoder::new(input)),
        "zlib" => Box::new(flate2::read::ZlibDecoder::new(input)),
        "deflate-raw" => Box::new(flate2::read::DeflateDecoder::new(input)),
        "zstd" => match zstd::stream::read::Decoder::new(input) {
            Ok(value) => Box::new(value),
            Err(error) => return ResultValue::error(format!("zstd decompression failed: {error}")),
        },
        _ => {
            return ResultValue::error(
                "unknown compression format; auto-detection is not supported",
            );
        }
    };
    let maximum = output_limit.min(work_limit).saturating_add(1);
    let mut data = Vec::new();
    match reader.take(maximum as u64).read_to_end(&mut data) {
        Err(error) => ResultValue::error(format!("decompression failed: {error}")),
        Ok(_) if data.len() > output_limit || data.len() > work_limit => {
            ResultValue::limit("decompression output or work limit exceeded")
        }
        Ok(_)
            if !input.is_empty()
                && data.len() as u128 > input.len() as u128 * ratio_limit as u128 =>
        {
            ResultValue::limit("decompression ratio limit exceeded")
        }
        Ok(_) => ResultValue {
            data,
            ..ResultValue::default()
        },
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
        .map_err(|_| ResultValue::error("deadline must be non-negative milliseconds"))?;
    Ok(Duration::from_millis(value))
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
pub fn tcp_connect(address: &str, deadline_ms: i128, cancelled: bool) -> ResultValue {
    if cancelled {
        return ResultValue::error("operation cancelled");
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
            let _ = stream.set_read_timeout(Some(duration));
            let _ = stream.set_write_timeout(Some(duration));
            ResultValue {
                capability: Some(Capability(Arc::new(CapabilityInner::Tcp(Mutex::new(
                    Some(stream),
                ))))),
                ..ResultValue::default()
            }
        }
        Err(error) => ResultValue::error(format!("TCP connect failed: {error}")),
    }
}
pub fn tcp_accept(listener: &Capability, deadline_ms: i128, cancelled: bool) -> ResultValue {
    if cancelled {
        return ResultValue::error("operation cancelled");
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
        if cancelled {
            return ResultValue::error("operation cancelled");
        }
        match listener.accept() {
            Ok((stream, peer)) => {
                let _ = listener.set_nonblocking(false);
                let _ = stream.set_read_timeout(Some(duration));
                let _ = stream.set_write_timeout(Some(duration));
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
                std::thread::sleep(Duration::from_millis(1))
            }
            Err(error) => {
                let _ = listener.set_nonblocking(false);
                return ResultValue::error(format!("TCP accept failed: {error}"));
            }
        }
    }
}
pub fn tcp_read(
    stream: &Capability,
    limit: i128,
    deadline_ms: i128,
    cancelled: bool,
) -> ResultValue {
    if cancelled {
        return ResultValue::error("operation cancelled");
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
    let _ = stream.set_read_timeout(Some(duration));
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
        Err(error) => ResultValue::error(format!("TCP read failed: {error}")),
    }
}
pub fn tcp_write(
    stream: &Capability,
    data: &[u8],
    deadline_ms: i128,
    cancelled: bool,
) -> ResultValue {
    if cancelled {
        return ResultValue::error("operation cancelled");
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
    let _ = stream.set_write_timeout(Some(duration));
    match stream.write(data) {
        Ok(count) => ResultValue {
            number: count as i128,
            ..ResultValue::default()
        },
        Err(error) => ResultValue::error(format!("TCP write failed: {error}")),
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
pub fn udp_send_to(
    socket: &Capability,
    data: &[u8],
    address: &str,
    deadline_ms: i128,
    cancelled: bool,
) -> ResultValue {
    if cancelled {
        return ResultValue::error("operation cancelled");
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
    let _ = socket.set_write_timeout(Some(duration));
    match socket.send_to(data, address) {
        Ok(count) => ResultValue {
            number: count as i128,
            ..ResultValue::default()
        },
        Err(error) => ResultValue::error(format!("UDP send failed: {error}")),
    }
}
pub fn udp_receive_from(
    socket: &Capability,
    limit: i128,
    deadline_ms: i128,
    cancelled: bool,
) -> ResultValue {
    if cancelled {
        return ResultValue::error("operation cancelled");
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
    let _ = socket.set_read_timeout(Some(duration));
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
        Err(error) => ResultValue::error(format!("UDP receive failed: {error}")),
    }
}

pub fn dns_lookup(host: &str, port: i128, deadline_ms: i128, cancelled: bool) -> ResultValue {
    if cancelled {
        return ResultValue::error("operation cancelled");
    }
    if timeout(deadline_ms).is_err() {
        return ResultValue::error("deadline must be non-negative milliseconds");
    }
    let Ok(port) = u16::try_from(port) else {
        return ResultValue::error("port must be in 0..=65535");
    };
    match (host, port).to_socket_addrs() {
        Ok(values) => ResultValue {
            entries: values.map(|value| value.ip().to_string()).collect(),
            number: 0,
            detail: "ttl-unavailable".to_owned(),
            ..ResultValue::default()
        },
        Err(error) => ResultValue::error(format!("DNS lookup failed: {error}")),
    }
}

pub fn tls_client(
    stream: &Capability,
    server_name: &str,
    deadline_ms: i128,
    cancelled: bool,
) -> ResultValue {
    if cancelled {
        return ResultValue::error("operation cancelled");
    }
    let duration = match timeout(deadline_ms) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return ResultValue::error("TLS requires a TCP stream");
    };
    let guard = stream.lock().expect("stream lock poisoned");
    let Some(tcp) = guard.as_ref() else {
        return ResultValue::error("stream is closed");
    };
    let Ok(tcp) = tcp.try_clone() else {
        return ResultValue::error("cannot clone TCP stream for TLS");
    };
    let _ = tcp.set_read_timeout(Some(duration));
    let _ = tcp.set_write_timeout(Some(duration));
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
        Err(error) => ResultValue::error(format!("TLS handshake failed: {error}")),
    }
}
pub fn tls_read(
    stream: &Capability,
    limit: i128,
    deadline_ms: i128,
    cancelled: bool,
) -> ResultValue {
    if cancelled {
        return ResultValue::error("operation cancelled");
    }
    let size = match count(limit, "TLS read limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let _ = match timeout(deadline_ms) {
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
        Err(error) => ResultValue::error(format!("TLS read failed: {error}")),
    }
}
pub fn tls_write(
    stream: &Capability,
    data: &[u8],
    deadline_ms: i128,
    cancelled: bool,
) -> ResultValue {
    if cancelled {
        return ResultValue::error("operation cancelled");
    }
    let _ = match timeout(deadline_ms) {
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
    match stream.write(data) {
        Ok(count) => ResultValue {
            number: count as i128,
            ..ResultValue::default()
        },
        Err(error) => ResultValue::error(format!("TLS write failed: {error}")),
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
        let client = tcp_connect(&address, 1_000, false);
        assert!(!client.failed);
        let server = tcp_accept(listener.capability.as_ref().unwrap(), 1_000, false);
        assert!(!server.failed);
        let sent = tcp_write(
            client.capability.as_ref().unwrap(),
            b"terrane",
            1_000,
            false,
        );
        assert_eq!(sent.number, 7);
        let received = tcp_read(server.capability.as_ref().unwrap(), 7, 1_000, false);
        assert_eq!(received.data, b"terrane");
    }

    #[test]
    fn oversized_udp_datagram_reports_truncation() {
        let receiver = udp_bind("127.0.0.1:0");
        let sender = udp_bind("127.0.0.1:0");
        assert!(!receiver.failed && !sender.failed);
        let sent = udp_send_to(
            sender.capability.as_ref().unwrap(),
            b"oversized",
            &receiver.text,
            1_000,
            false,
        );
        assert_eq!(sent.number, 9);
        let datagram = udp_receive_from(receiver.capability.as_ref().unwrap(), 4, 1_000, false);
        assert!(datagram.truncated);
        assert_eq!(datagram.data, b"over");
    }
}
