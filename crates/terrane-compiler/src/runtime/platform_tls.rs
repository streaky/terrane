pub fn terrane_platform_tls_client(stream: &TerranePlatformCapability, server: String, deadline: terrane_int_support::Int, cancellation: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::tls_client(stream, &server, terrane_platform_i128!(deadline, "TLS deadline"), cancellation) }
pub fn terrane_platform_tls_read(stream: &TerranePlatformCapability, limit: terrane_int_support::Int, deadline: terrane_int_support::Int, cancellation: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::tls_read(stream, terrane_platform_i128!(limit, "TLS read limit"), terrane_platform_i128!(deadline, "TLS read deadline"), cancellation) }
pub fn terrane_platform_tls_write(stream: &TerranePlatformCapability, data: Vec<u8>, deadline: terrane_int_support::Int, cancellation: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::tls_write(stream, &data, terrane_platform_i128!(deadline, "TLS write deadline"), cancellation) }
pub fn terrane_platform_tls_shutdown(stream: &TerranePlatformCapability, deadline: terrane_int_support::Int, cancellation: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::tls_shutdown(stream, terrane_platform_i128!(deadline, "TLS shutdown deadline"), cancellation) }

async fn terrane_platform_tls_blocking(
    work: impl FnOnce() -> TerranePlatformResult + Send + 'static,
) -> TerranePlatformResult {
    tokio::task::spawn_blocking(work)
        .await
        .expect("delegated TLS operation must not panic")
}

pub async fn terrane_platform_tls_client_async(
    stream: &TerranePlatformCapability,
    server: String,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let stream = stream.clone();
    let cancellation = cancellation.clone();
    terrane_platform_tls_blocking(move || {
        terrane_platform_tls_client(&stream, server, deadline, &cancellation)
    })
    .await
}

pub async fn terrane_platform_tls_read_async(
    stream: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let stream = stream.clone();
    let cancellation = cancellation.clone();
    terrane_platform_tls_blocking(move || {
        terrane_platform_tls_read(&stream, limit, deadline, &cancellation)
    })
    .await
}

pub async fn terrane_platform_tls_write_async(
    stream: &TerranePlatformCapability,
    data: Vec<u8>,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let stream = stream.clone();
    let cancellation = cancellation.clone();
    terrane_platform_tls_blocking(move || {
        terrane_platform_tls_write(&stream, data, deadline, &cancellation)
    })
    .await
}

pub async fn terrane_platform_tls_shutdown_async(
    stream: &TerranePlatformCapability,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let stream = stream.clone();
    let cancellation = cancellation.clone();
    terrane_platform_tls_blocking(move || {
        terrane_platform_tls_shutdown(&stream, deadline, &cancellation)
    })
    .await
}
