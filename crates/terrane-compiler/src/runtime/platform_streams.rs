pub async fn terrane_platform_read_async(
    handle: &TerranePlatformStreamHandle,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformReadResult {
    if let Some(error) = terrane_platform_stream_options(&deadline, cancellation) {
        return TerranePlatformReadResult {
            data: Vec::new(),
            completed: 0_i64.into(),
            end: false,
            failed: error.failed,
            message: error.message,
            deadline_exceeded: error.deadline_exceeded,
        };
    }
    let handle = handle.clone();
    // Blocking delegates cannot be safely abandoned while they own a descriptor or read buffer.
    let _wait = __terrane_finally_guard();
    tokio::task::spawn_blocking(move || terrane_platform_read(&handle, limit))
        .await
        .expect("delegated stream read must not panic")
}

pub fn terrane_platform_buffer_create(
    capacity: &terrane_int_support::Int,
) -> TerranePlatformResult {
    let capacity = match terrane_platform_i128(capacity, "read buffer capacity") {
        Ok(value) => value,
        Err(error) => return error,
    };
    terrane_platform_support::buffer_create(capacity)
}

pub fn terrane_platform_buffer_clear(buffer: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::buffer_clear(buffer)
}

pub fn terrane_platform_buffer_capacity(
    buffer: &TerranePlatformCapability,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(
        terrane_platform_support::platform_buffer_capacity(buffer).number,
    )
}

pub fn terrane_platform_buffer_completed(
    buffer: &TerranePlatformCapability,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(
        terrane_platform_support::platform_buffer_completed(buffer).number,
    )
}

pub fn terrane_platform_buffer_filled_bytes(
    buffer: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::buffer_filled_bytes(buffer)
}

pub fn terrane_platform_buffer_filled_text(
    buffer: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::buffer_filled_text(buffer)
}

pub async fn terrane_platform_tcp_read_into_async(
    stream: &TerranePlatformCapability,
    buffer: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    let limit = match terrane_platform_i128(&limit, "TCP read limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    terrane_platform_support::tcp_read_into_async(
        stream,
        buffer,
        limit,
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}

pub async fn terrane_platform_tls_read_into_async(
    stream: &TerranePlatformCapability,
    buffer: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    let limit = match terrane_platform_i128(&limit, "TLS read limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    terrane_platform_support::tls_read_into_async(
        stream,
        buffer,
        limit,
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}

pub async fn terrane_platform_stream_read_into_async(
    handle: &TerranePlatformStreamHandle,
    buffer: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    if let Some(error) = terrane_platform_stream_options(&deadline, cancellation) {
        return error;
    }
    let Some(limit) = limit.as_usize() else {
        return TerranePlatformResult::error(
            "stream read count is outside the supported size range",
        );
    };
    let handle = handle.clone();
    let buffer = buffer.clone();
    let _wait = __terrane_finally_guard();
    tokio::task::spawn_blocking(move || {
        let mut result = terrane_platform_support::buffer_read_with(&buffer, |storage| {
            if limit > storage.len() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "stream read count exceeds read buffer capacity",
                ));
            }
            if limit == 0 {
                return Ok(0);
            }
            terrane_stream_abi::read_into(handle.abi_handle(), &mut storage[..limit])
        });
        if limit == 0 && !result.failed {
            result.flag = false;
        }
        result
    })
    .await
    .expect("delegated stream buffer read must not panic")
}

pub async fn terrane_platform_write_async(
    handle: &TerranePlatformStreamHandle,
    data: Vec<u8>,
    offset: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformWriteResult {
    if let Some(error) = terrane_platform_stream_options(&deadline, cancellation) {
        return TerranePlatformWriteResult {
            data,
            completed: 0_i64.into(),
            failed: error.failed,
            message: error.message,
            deadline_exceeded: error.deadline_exceeded,
        };
    }
    let handle = handle.clone();
    let _wait = __terrane_finally_guard();
    tokio::task::spawn_blocking(move || {
        let mut result = terrane_platform_write(&handle, &data, offset);
        result.data = data;
        result
    })
    .await
    .expect("delegated stream write must not panic")
}

pub fn terrane_platform_buffer_close(buffer: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::close(buffer)
}

fn terrane_platform_stream_options(
    deadline: &terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> Option<TerranePlatformResult> {
    if let Some(token) = cancellation {
        let result = terrane_platform_support::check_cancellation(Some(token));
        if result.failed {
            return Some(result);
        }
    }
    match terrane_platform_i128(deadline, "stream deadline") {
        Ok(-1) => None,
        Ok(0) => Some(TerranePlatformResult {
            failed: true,
            deadline_exceeded: true,
            message: "stream deadline exceeded".to_owned(),
            ..TerranePlatformResult::default()
        }),
        Ok(value) if value < -1 => Some(TerranePlatformResult::error(
            "stream deadline is outside the supported range",
        )),
        // Blocking standard-stream calls cannot be interrupted once delegated. Treat a live
        // deadline as preflight-only and join completion rather than abandoning the task.
        Ok(_) => None,
        Err(error) => Some(error),
    }
}
