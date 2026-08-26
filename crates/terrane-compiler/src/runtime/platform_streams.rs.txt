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
struct TerranePlatformReadResult {
    data: Vec<u8>,
    completed: terrane_int_support::Int,
    end: bool,
    failed: bool,
    message: String,
}

#[derive(Clone)]
struct TerranePlatformWriteResult {
    completed: terrane_int_support::Int,
    failed: bool,
    message: String,
}

#[derive(Clone)]
struct TerranePlatformUnitResult {
    failed: bool,
    message: String,
}

fn terrane_platform_acquire_stdin() -> TerranePlatformStreamHandle {
    TerranePlatformStreamHandle::new(terrane_stream_abi::acquire_stdin())
}

fn terrane_platform_acquire_stdout() -> TerranePlatformStreamHandle {
    TerranePlatformStreamHandle::new(terrane_stream_abi::acquire_stdout())
}

fn terrane_platform_acquire_stderr() -> TerranePlatformStreamHandle {
    TerranePlatformStreamHandle::new(terrane_stream_abi::acquire_stderr())
}

fn terrane_platform_read(
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
        };
    };
    match terrane_stream_abi::read(handle.abi_handle(), limit) {
        Ok(outcome) => TerranePlatformReadResult {
            completed: terrane_int_support::Int::from(outcome.data.len() as i128),
            data: outcome.data,
            end: outcome.end,
            failed: false,
            message: String::new(),
        },
        Err(error) => TerranePlatformReadResult {
            data: Vec::new(),
            completed: terrane_int_support::Int::from(0_i64),
            end: false,
            failed: true,
            message: error.to_string(),
        },
    }
}

fn terrane_platform_write(
    handle: &TerranePlatformStreamHandle,
    data: &[u8],
    offset: terrane_int_support::Int,
) -> TerranePlatformWriteResult {
    let Some(offset) = offset.as_usize().filter(|offset| *offset <= data.len()) else {
        return TerranePlatformWriteResult {
            completed: terrane_int_support::Int::from(0_i64),
            failed: true,
            message: "stream write offset is outside the buffer".to_owned(),
        };
    };
    match terrane_stream_abi::write(handle.abi_handle(), &data[offset..]) {
        Ok(completed) => TerranePlatformWriteResult {
            completed: terrane_int_support::Int::from(completed as i128),
            failed: false,
            message: String::new(),
        },
        Err(error) => TerranePlatformWriteResult {
            completed: terrane_int_support::Int::from(0_i64),
            failed: true,
            message: error.to_string(),
        },
    }
}

fn terrane_platform_flush(handle: &TerranePlatformStreamHandle) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::flush(handle.abi_handle()))
}

fn terrane_platform_sync_data(handle: &TerranePlatformStreamHandle) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_data(handle.abi_handle()))
}

fn terrane_platform_sync_all(handle: &TerranePlatformStreamHandle) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_all(handle.abi_handle()))
}

fn terrane_platform_close(handle: &TerranePlatformStreamHandle) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::close(handle.abi_handle()))
}

// Resource ownership is compiler-inferred in Terrane. The final owning wrapper releases the
// process handle; adapters may share this compiler-owned indirection without exposing aliases.
fn terrane_platform_release(handle: &TerranePlatformStreamHandle) -> TerranePlatformUnitResult {
    if std::sync::Arc::strong_count(&handle.0) == 1 {
        terrane_platform_close(handle)
    } else {
        TerranePlatformUnitResult {
            failed: false,
            message: String::new(),
        }
    }
}

fn terrane_platform_unit(result: std::io::Result<()>) -> TerranePlatformUnitResult {
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
