pub fn terrane_platform_acquire_stdin() -> TerranePlatformStreamHandle {
    terrane_stream_abi::acquire_stdin().into()
}

pub fn terrane_platform_acquire_stdout() -> TerranePlatformStreamHandle {
    terrane_stream_abi::acquire_stdout().into()
}

pub fn terrane_platform_acquire_stderr() -> TerranePlatformStreamHandle {
    terrane_stream_abi::acquire_stderr().into()
}
