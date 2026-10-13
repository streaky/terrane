// `Arc` keeps a host handle alive while a consuming Terrane adapter transfers it between
// generated wrappers. It is lowering machinery, not shared ownership in Terrane: static move
// analysis still permits exactly one source-level owner.
#[derive(Clone)]
pub struct TerranePlatformStreamHandle(std::sync::Arc<i64>);
impl Default for TerranePlatformStreamHandle {
    fn default() -> Self {
        Self(std::sync::Arc::new(0))
    }
}

impl TerranePlatformStreamHandle {
    fn new(handle: terrane_stream_abi::StreamHandle) -> Self {
        Self(std::sync::Arc::new(handle.id()))
    }

    fn abi_handle(&self) -> terrane_stream_abi::StreamHandle {
        terrane_stream_abi::StreamHandle::from_id(*self.0)
    }
}

#[derive(Clone)]
pub struct TerranePlatformReadResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
    pub deadline_exceeded: bool,
}

#[derive(Clone)]
pub struct TerranePlatformWriteResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub failed: bool,
    pub message: String,
    pub deadline_exceeded: bool,
}

#[derive(Clone)]
pub struct TerranePlatformUnitResult {
    pub failed: bool,
    pub message: String,
}

pub fn terrane_platform_read(
    handle: &TerranePlatformStreamHandle,
    limit: terrane_int_support::Int,
) -> TerranePlatformReadResult {
    let Some(limit) = limit.as_usize() else {
        return TerranePlatformReadResult {
            data: Vec::new(),
            completed: terrane_int_support::Int::from(0_i64),
            end: false,
            failed: true,
            message: "stream read count is outside the supported size range".to_owned(),
            deadline_exceeded: false,
        };
    };
    match terrane_stream_abi::read(handle.abi_handle(), limit) {
        Ok(outcome) => TerranePlatformReadResult {
            completed: terrane_int_support::Int::from(outcome.data.len() as i128),
            data: outcome.data,
            end: outcome.end,
            failed: false,
            message: String::new(),
            deadline_exceeded: false,
        },
        Err(error) => TerranePlatformReadResult {
            data: Vec::new(),
            completed: terrane_int_support::Int::from(0_i64),
            end: false,
            failed: true,
            message: error.to_string(),
            deadline_exceeded: false,
        },
    }
}

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

pub fn terrane_platform_write(
    handle: &TerranePlatformStreamHandle,
    data: &[u8],
    offset: terrane_int_support::Int,
) -> TerranePlatformWriteResult {
    let Some(offset) = offset.as_usize().filter(|offset| *offset <= data.len()) else {
        return TerranePlatformWriteResult {
            data: Vec::new(),
            completed: terrane_int_support::Int::from(0_i64),
            failed: true,
            message: "stream write offset is outside the buffer".to_owned(),
            deadline_exceeded: false,
        };
    };
    match terrane_stream_abi::write(handle.abi_handle(), &data[offset..]) {
        Ok(completed) => TerranePlatformWriteResult {
            data: Vec::new(),
            completed: terrane_int_support::Int::from(completed as i128),
            failed: false,
            message: String::new(),
            deadline_exceeded: false,
        },
        Err(error) => TerranePlatformWriteResult {
            data: Vec::new(),
            completed: terrane_int_support::Int::from(0_i64),
            failed: true,
            message: error.to_string(),
            deadline_exceeded: false,
        },
    }
}

pub fn terrane_platform_flush(handle: &TerranePlatformStreamHandle) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::flush(handle.abi_handle()))
}

pub fn terrane_platform_sync_data(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_data(handle.abi_handle()))
}

pub fn terrane_platform_sync_all(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_all(handle.abi_handle()))
}

pub fn terrane_platform_close(handle: &TerranePlatformStreamHandle) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::close(handle.abi_handle()))
}

// Resource ownership is compiler-inferred in Terrane. The final generated wrapper releases the
// process handle. The Arc count only distinguishes an in-progress adapter transfer; it does not
// make the handle copyable or observable as shared ownership in Terrane.
pub fn terrane_platform_release(handle: &TerranePlatformStreamHandle) -> TerranePlatformUnitResult {
    if std::sync::Arc::strong_count(&handle.0) == 1 {
        terrane_platform_unit(terrane_stream_abi::release(handle.abi_handle()))
    } else {
        TerranePlatformUnitResult {
            failed: false,
            message: String::new(),
        }
    }
}

pub fn terrane_platform_unit(result: std::io::Result<()>) -> TerranePlatformUnitResult {
    match result {
        Ok(()) => TerranePlatformUnitResult {
            failed: false,
            message: String::new(),
        },
        Err(error) => TerranePlatformUnitResult {
            failed: true,
            message: error.to_string(),
        },
    }
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
    terrane_int_support::Int::from(terrane_platform_support::buffer_capacity(buffer).number)
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
        return Some(TerranePlatformResult::error(
            "cancellation of blocking standard-stream I/O is unsupported",
        ));
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
        Ok(_) => Some(TerranePlatformResult::error(
            "deadlines for blocking standard-stream I/O are unsupported",
        )),
        Err(error) => Some(error),
    }
}
