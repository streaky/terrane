pub fn terrane_platform_parse_ip(text: String) -> TerranePlatformResult {
    terrane_platform_support::parse_ip(&text)
}
pub fn terrane_platform_parse_host_name(text: String) -> TerranePlatformResult {
    terrane_platform_support::parse_host_name(&text)
}
pub fn terrane_platform_parse_socket(
    ip: &String,
    port: &terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::parse_socket(ip, terrane_platform_i128!(port, "socket port"))
}
pub fn terrane_platform_parse_socket_text(text: String) -> TerranePlatformResult {
    terrane_platform_support::parse_socket_text(&text)
}
fn terrane_platform_tcp_listener_into_native(
    listener: &TerranePlatformCapability,
) -> std::io::Result<tokio::net::TcpListener> {
    terrane_platform_support::into_native_tcp_listener(listener)
}
fn terrane_platform_tcp_stream_into_native(
    stream: &TerranePlatformCapability,
) -> std::io::Result<tokio::net::TcpStream> {
    terrane_platform_support::into_native_tcp_stream(stream)
}
fn terrane_platform_native_result_failed<T>(result: &std::io::Result<T>) -> bool {
    result.is_err()
}
fn terrane_platform_native_result_message<T>(result: &std::io::Result<T>) -> String {
    result
        .as_ref()
        .err()
        .map_or_else(String::new, ToString::to_string)
}
fn terrane_platform_native_result_value<T>(result: std::io::Result<T>) -> T {
    result.expect("native transfer result was checked")
}

pub async fn terrane_platform_tcp_listen_async(
    address: String,
    backlog: terrane_int_support::Int,
    reuse_address: bool,
    accepted_no_delay: bool,
    accepted_ttl: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_listen_async(
        &address,
        terrane_platform_i128!(backlog, "TCP backlog"),
        reuse_address,
        accepted_no_delay,
        terrane_platform_i128!(accepted_ttl, "accepted TCP TTL"),
    )
    .await
}

pub fn terrane_platform_tcp_bind(
    address: String,
    backlog: terrane_int_support::Int,
    reuse_address: bool,
    accepted_no_delay: bool,
    accepted_ttl: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_bind(
        &address,
        terrane_platform_i128!(backlog, "TCP backlog"),
        reuse_address,
        accepted_no_delay,
        terrane_platform_i128!(accepted_ttl, "accepted TCP TTL"),
    )
}
pub fn terrane_platform_tcp_connect(
    address: String,
    no_delay: bool,
    ttl: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_connect(
        &address,
        no_delay,
        terrane_platform_i128!(ttl, "TCP TTL"),
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
}
pub fn terrane_platform_tcp_connect_host(
    host: String,
    port: terrane_int_support::Int,
    no_delay: bool,
    ttl: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_connect_host(
        &host,
        terrane_platform_i128!(port, "TCP host port"),
        no_delay,
        terrane_platform_i128!(ttl, "TCP TTL"),
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
}
pub fn terrane_platform_tcp_accept(
    listener: &TerranePlatformCapability,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_accept(
        listener,
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
}
pub fn terrane_platform_tcp_read(
    stream: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_read(
        stream,
        terrane_platform_i128!(limit, "TCP read limit"),
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
}
pub fn terrane_platform_tcp_write(
    stream: &TerranePlatformCapability,
    data: Vec<u8>,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_write(
        stream,
        &data,
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
}
pub fn terrane_platform_tcp_shutdown(
    stream: &TerranePlatformCapability,
    direction: String,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_shutdown(stream, &direction)
}
pub fn terrane_platform_tcp_configure(
    stream: &TerranePlatformCapability,
    no_delay: bool,
    ttl: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_configure(
        stream,
        no_delay,
        terrane_platform_i128!(ttl, "TCP TTL"),
    )
}
pub fn terrane_platform_udp_bind(address: String) -> TerranePlatformResult {
    terrane_platform_support::udp_bind(&address)
}
pub fn terrane_platform_udp_configure(
    socket: &TerranePlatformCapability,
    broadcast: bool,
    ttl: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::udp_configure(
        socket,
        broadcast,
        terrane_platform_i128!(ttl, "UDP TTL"),
    )
}
pub fn terrane_platform_udp_send_to(
    socket: &TerranePlatformCapability,
    data: Vec<u8>,
    address: String,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::udp_send_to(
        socket,
        &data,
        &address,
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
}
pub fn terrane_platform_udp_receive_from(
    socket: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::udp_receive_from(
        socket,
        terrane_platform_i128!(limit, "UDP receive limit"),
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
}
pub fn terrane_platform_dns_lookup(
    host: String,
    port: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::dns_lookup(
        &host,
        terrane_platform_i128!(port, "DNS port"),
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
}
pub fn terrane_platform_capability_close(
    capability: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::close(capability)
}

async fn terrane_platform_network_blocking(
    work: impl FnOnce() -> TerranePlatformResult + Send + 'static,
) -> TerranePlatformResult {
    tokio::task::spawn_blocking(work)
        .await
        .expect("delegated network operation must not panic")
}

pub async fn terrane_platform_tcp_connect_async(
    address: String,
    no_delay: bool,
    ttl: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_connect_async(
        &address,
        no_delay,
        terrane_platform_i128!(ttl, "TCP TTL"),
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}

pub async fn terrane_platform_tcp_connect_host_async(
    host: String,
    port: terrane_int_support::Int,
    no_delay: bool,
    ttl: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_connect_host_async(
        &host,
        terrane_platform_i128!(port, "TCP host port"),
        no_delay,
        terrane_platform_i128!(ttl, "TCP TTL"),
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}

pub async fn terrane_platform_tcp_accept_async(
    listener: &TerranePlatformCapability,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_accept_async(
        listener,
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}

pub async fn terrane_platform_tcp_read_async(
    stream: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_read_async(
        stream,
        terrane_platform_i128!(limit, "TCP read limit"),
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}

pub async fn terrane_platform_tcp_write_async(
    stream: &TerranePlatformCapability,
    data: &[u8],
    offset: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    let Some(offset) = offset.as_usize().filter(|offset| *offset <= data.len()) else {
        return TerranePlatformResult::error("TCP write offset is outside the buffer");
    };
    terrane_platform_support::tcp_write_async(
        stream,
        &data[offset..],
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}

pub async fn terrane_platform_udp_send_to_async(
    socket: &TerranePlatformCapability,
    data: Vec<u8>,
    address: String,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::udp_send_to_async(
        socket,
        &data,
        &address,
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}

pub async fn terrane_platform_udp_receive_from_async(
    socket: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    terrane_platform_support::udp_receive_from_async(
        socket,
        terrane_platform_i128!(limit, "UDP receive limit"),
        terrane_platform_timeout_nanos(deadline),
        cancellation,
    )
    .await
}

pub async fn terrane_platform_dns_lookup_async(
    host: String,
    port: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    let cancellation = cancellation.cloned();
    terrane_platform_network_blocking(move || {
        terrane_platform_dns_lookup(host, port, deadline, cancellation.as_ref())
    })
    .await
}
