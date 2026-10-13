use super::{Capability, CapabilityInner, ResultValue, count, io_error, readiness_operation};

pub(super) fn take_bytes(buffer: &Capability) -> ResultValue {
    let CapabilityInner::Buffer(buffer) = buffer.0.as_ref() else {
        return ResultValue::error("capability is not a read buffer");
    };
    match buffer.try_lock() {
        Ok(mut buffer) => {
            if buffer.closed {
                return ResultValue::error("read buffer is closed");
            }
            let filled = buffer.filled;
            let bytes = buffer.bytes[..filled].to_vec();
            buffer.filled = 0;
            ResultValue {
                number: bytes.len() as i128,
                data: bytes,
                ..ResultValue::default()
            }
        }
        Err(_) => ResultValue::error("read buffer is busy"),
    }
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
