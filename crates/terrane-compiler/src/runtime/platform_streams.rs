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
}

#[derive(Clone)]
pub struct TerranePlatformWriteResult {
    pub completed: terrane_int_support::Int,
    pub failed: bool,
    pub message: String,
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

pub fn terrane_platform_write(
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

pub fn terrane_platform_flush(handle: &TerranePlatformStreamHandle) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::flush(handle.abi_handle()))
}

pub fn terrane_platform_sync_data(handle: &TerranePlatformStreamHandle) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_data(handle.abi_handle()))
}

pub fn terrane_platform_sync_all(handle: &TerranePlatformStreamHandle) -> TerranePlatformUnitResult {
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
