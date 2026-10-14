pub fn terrane_platform_tls_client(
    stream: &TerranePlatformCapability,
    server: String,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tls_client(
        stream,
        &server,
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
}
pub fn terrane_platform_tls_read(
    stream: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tls_read(
        stream,
        terrane_platform_i128!(limit, "TLS read limit"),
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
}
pub fn terrane_platform_tls_write(
    stream: &TerranePlatformCapability,
    data: Vec<u8>,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tls_write(
        stream,
        &data,
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
}
pub fn terrane_platform_tls_shutdown(
    stream: &TerranePlatformCapability,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tls_shutdown(
        stream,
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
}

pub async fn terrane_platform_tls_client_async(
    stream: &TerranePlatformCapability,
    server: String,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tls_client_async(
        stream,
        &server,
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}

pub async fn terrane_platform_tls_read_async(
    stream: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tls_read_async(
        stream,
        terrane_platform_i128!(limit, "TLS read limit"),
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}

pub async fn terrane_platform_tls_write_async(
    stream: &TerranePlatformCapability,
    data: &[u8],
    offset: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    let Some(offset) = offset.as_usize().filter(|offset| *offset <= data.len()) else {
        return TerranePlatformResult::error("TLS write offset is outside the buffer");
    };
    terrane_platform_support::tls_write_async(
        stream,
        &data[offset..],
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}

pub async fn terrane_platform_tls_shutdown_async(
    stream: &TerranePlatformCapability,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tls_shutdown_async(
        stream,
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}
