use super::ReadBuffer;
use super::networking_state;
use super::{Capability, CapabilityInner, ResultValue, count, io_error, readiness_operation};
use std::sync::Arc;

pub fn buffer_create(capacity: i128) -> ResultValue {
    let capacity = match count(capacity, "buffer capacity") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let mut bytes = Vec::new();
    if let Err(error) = bytes.try_reserve_exact(capacity) {
        return ResultValue::limit(format!("cannot allocate read buffer: {error}"));
    }
    bytes.resize(capacity, 0);
    ResultValue {
        capability: Some(Capability(Arc::new(CapabilityInner::Buffer(
            tokio::sync::Mutex::new(ReadBuffer {
                bytes,
                filled: 0,
                closed: false,
            }),
        )))),
        ..ResultValue::default()
    }
}
pub(super) fn buffer_capacity(buffer: &Capability) -> ResultValue {
    let CapabilityInner::Buffer(buffer) = buffer.0.as_ref() else {
        return ResultValue::error("capability is not a read buffer");
    };
    match buffer.try_lock() {
        Ok(buffer) if buffer.closed => ResultValue::error("read buffer is closed"),
        Ok(buffer) => ResultValue {
            number: buffer.bytes.len() as i128,
            ..ResultValue::default()
        },
        Err(_) => ResultValue::error("read buffer is busy"),
    }
}

pub(super) fn buffer_completed(buffer: &Capability) -> ResultValue {
    let CapabilityInner::Buffer(buffer) = buffer.0.as_ref() else {
        return ResultValue::error("capability is not a read buffer");
    };
    match buffer.try_lock() {
        Ok(buffer) if buffer.closed => ResultValue::error("read buffer is closed"),
        Ok(buffer) => ResultValue {
            number: buffer.filled as i128,
            ..ResultValue::default()
        },
        Err(_) => ResultValue::error("read buffer is busy"),
    }
}

pub(super) fn buffer_clear(buffer: &Capability) -> ResultValue {
    let CapabilityInner::Buffer(buffer) = buffer.0.as_ref() else {
        return ResultValue::error("capability is not a read buffer");
    };
    match buffer.try_lock() {
        Ok(buffer) if buffer.closed => ResultValue::error("read buffer is closed"),
        Ok(mut buffer) => {
            buffer.filled = 0;
            ResultValue::default()
        }
        Err(_) => ResultValue::error("read buffer is busy"),
    }
}

pub(super) fn buffer_filled_bytes(buffer: &Capability) -> ResultValue {
    let CapabilityInner::Buffer(buffer) = buffer.0.as_ref() else {
        return ResultValue::error("capability is not a read buffer");
    };
    match buffer.try_lock() {
        Ok(buffer) if buffer.closed => ResultValue::error("read buffer is closed"),
        Ok(buffer) => ResultValue {
            data: buffer.bytes[..buffer.filled].to_vec(),
            ..ResultValue::default()
        },
        Err(_) => ResultValue::error("read buffer is busy"),
    }
}

pub(super) fn buffer_filled_text(buffer: &Capability) -> ResultValue {
    let CapabilityInner::Buffer(buffer) = buffer.0.as_ref() else {
        return ResultValue::error("capability is not a read buffer");
    };
    match buffer.try_lock() {
        Ok(buffer) if buffer.closed => ResultValue::error("read buffer is closed"),
        Ok(buffer) => match std::str::from_utf8(&buffer.bytes[..buffer.filled]) {
            Ok(text) => ResultValue {
                text: text.to_owned(),
                ..ResultValue::default()
            },
            Err(_) => ResultValue::error("read buffer contents are not valid UTF-8"),
        },
        Err(_) => ResultValue::error("read buffer is busy"),
    }
}

pub(super) fn buffer_read_with(
    buffer: &Capability,
    read: impl FnOnce(&mut [u8]) -> std::io::Result<usize>,
) -> ResultValue {
    let CapabilityInner::Buffer(buffer) = buffer.0.as_ref() else {
        return ResultValue::error("capability is not a read buffer");
    };
    let Ok(mut buffer) = buffer.try_lock() else {
        return ResultValue::error("read buffer is busy");
    };
    if buffer.closed {
        return ResultValue::error("read buffer is closed");
    }
    let offset = buffer.filled;
    let remaining = buffer.bytes.len() - offset;
    match read(&mut buffer.bytes[offset..]) {
        Ok(filled) if filled <= remaining => {
            buffer.filled += filled;
            ResultValue {
                number: filled as i128,
                flag: filled == 0 && remaining != 0,
                ..ResultValue::default()
            }
        }
        Ok(_) => ResultValue::error("read operation returned a count larger than buffer capacity"),
        Err(error) => super::io_error("stream read", &error),
    }
}

pub(super) async fn tcp_read_into(
    stream: &Capability,
    buffer: &Capability,
    limit: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let CapabilityInner::Buffer(buffer) = buffer.0.as_ref() else {
        return ResultValue::error("capability is not a read buffer");
    };
    let mut buffer = buffer.lock().await;
    if buffer.closed {
        return ResultValue::error("read buffer is closed");
    }
    let offset = buffer.filled;
    let limit = match count(limit, "read limit") {
        Ok(limit) if limit <= buffer.bytes.len() - offset => limit,
        Ok(_) => return ResultValue::error("read limit exceeds remaining buffer capacity"),
        Err(error) => return error,
    };
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not a TCP stream");
    };
    let stream = match stream.readiness() {
        Ok(value) => value,
        Err(error) => return error,
    };
    if limit == 0 {
        return ResultValue::default();
    }
    readiness_operation(
        async {
            match networking_state::read(&stream, &mut buffer.bytes[offset..offset + limit]).await {
                Ok(completed) => {
                    buffer.filled += completed;
                    ResultValue {
                        number: completed as i128,
                        flag: completed == 0,
                        ..ResultValue::default()
                    }
                }
                Err(error) => super::io_error("TCP read", &error),
            }
        },
        deadline_nanos,
        "TCP read deadline exceeded",
        cancellation,
    )
    .await
}

pub(super) async fn tls_read_into(
    stream: &Capability,
    buffer: &Capability,
    limit: i128,
    deadline: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let CapabilityInner::AsyncTls(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not an asynchronous TLS stream");
    };
    let CapabilityInner::Buffer(buffer) = buffer.0.as_ref() else {
        return ResultValue::error("capability is not a read buffer");
    };
    let limit = match count(limit, "read limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    readiness_operation(
        async {
            use tokio::io::AsyncReadExt as _;
            let mut buffer = buffer.lock().await;
            if buffer.closed {
                return ResultValue::error("read buffer is closed");
            }
            let offset = buffer.filled;
            if limit > buffer.bytes.len() - offset {
                return ResultValue::error("read limit exceeds remaining buffer capacity");
            }
            if limit == 0 {
                return ResultValue::default();
            }
            let mut stream = stream.reader.lock().await;
            let Some(stream) = stream.as_mut() else {
                return ResultValue::error("TLS stream is closed");
            };
            match stream.read(&mut buffer.bytes[offset..offset + limit]).await {
                Ok(completed) => {
                    buffer.filled += completed;
                    ResultValue {
                        number: completed as i128,
                        flag: completed == 0,
                        ..ResultValue::default()
                    }
                }
                Err(error) => io_error("TLS read", &error),
            }
        },
        deadline,
        "TLS read deadline exceeded",
        cancellation,
    )
    .await
}
