pub fn terrane_platform_parse_ip(text: String) -> TerranePlatformResult { terrane_platform_support::parse_ip(&text) }
pub fn terrane_platform_parse_host_name(text: String) -> TerranePlatformResult { terrane_platform_support::parse_host_name(&text) }
pub fn terrane_platform_parse_socket(ip: &String, port: &terrane_int_support::Int) -> TerranePlatformResult { terrane_platform_support::parse_socket(ip, terrane_platform_i128!(port, "socket port")) }
pub fn terrane_platform_parse_socket_text(text: String) -> TerranePlatformResult { terrane_platform_support::parse_socket_text(&text) }
pub fn terrane_platform_tcp_bind(address: String) -> TerranePlatformResult { terrane_platform_support::tcp_bind(&address) }
pub fn terrane_platform_tcp_connect(address: String, deadline: terrane_int_support::Int, cancellation: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::tcp_connect(&address, terrane_platform_i128!(deadline, "TCP connect deadline"), cancellation) }
pub fn terrane_platform_tcp_connect_host(host: String, port: terrane_int_support::Int, deadline: terrane_int_support::Int, cancellation: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::tcp_connect_host(&host, terrane_platform_i128!(port, "TCP host port"), terrane_platform_i128!(deadline, "TCP host connect deadline"), cancellation) }
pub fn terrane_platform_tcp_accept(listener: &TerranePlatformCapability, deadline: terrane_int_support::Int, cancellation: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::tcp_accept(listener, terrane_platform_i128!(deadline, "TCP accept deadline"), cancellation) }
pub fn terrane_platform_tcp_read(stream: &TerranePlatformCapability, limit: terrane_int_support::Int, deadline: terrane_int_support::Int, cancellation: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::tcp_read(stream, terrane_platform_i128!(limit, "TCP read limit"), terrane_platform_i128!(deadline, "TCP read deadline"), cancellation) }
pub fn terrane_platform_tcp_write(stream: &TerranePlatformCapability, data: Vec<u8>, deadline: terrane_int_support::Int, cancellation: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::tcp_write(stream, &data, terrane_platform_i128!(deadline, "TCP write deadline"), cancellation) }
pub fn terrane_platform_tcp_shutdown(stream: &TerranePlatformCapability, direction: String) -> TerranePlatformResult { terrane_platform_support::tcp_shutdown(stream, &direction) }
pub fn terrane_platform_tcp_configure(stream: &TerranePlatformCapability, no_delay: bool, ttl: terrane_int_support::Int) -> TerranePlatformResult { terrane_platform_support::tcp_configure(stream, no_delay, terrane_platform_i128!(ttl, "TCP TTL")) }
pub fn terrane_platform_udp_bind(address: String) -> TerranePlatformResult { terrane_platform_support::udp_bind(&address) }
pub fn terrane_platform_udp_configure(socket: &TerranePlatformCapability, broadcast: bool, ttl: terrane_int_support::Int) -> TerranePlatformResult { terrane_platform_support::udp_configure(socket, broadcast, terrane_platform_i128!(ttl, "UDP TTL")) }
pub fn terrane_platform_udp_send_to(socket: &TerranePlatformCapability, data: Vec<u8>, address: String, deadline: terrane_int_support::Int, cancellation: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::udp_send_to(socket, &data, &address, terrane_platform_i128!(deadline, "UDP send deadline"), cancellation) }
pub fn terrane_platform_udp_receive_from(socket: &TerranePlatformCapability, limit: terrane_int_support::Int, deadline: terrane_int_support::Int, cancellation: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::udp_receive_from(socket, terrane_platform_i128!(limit, "UDP receive limit"), terrane_platform_i128!(deadline, "UDP receive deadline"), cancellation) }
pub fn terrane_platform_dns_lookup(host: String, port: terrane_int_support::Int, deadline: terrane_int_support::Int, cancellation: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::dns_lookup(&host, terrane_platform_i128!(port, "DNS port"), terrane_platform_i128!(deadline, "DNS deadline"), cancellation) }
pub fn terrane_platform_capability_close(capability: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::close(capability) }

async fn terrane_platform_network_blocking(
    work: impl FnOnce() -> TerranePlatformResult + Send + 'static,
) -> TerranePlatformResult {
    tokio::task::spawn_blocking(work)
        .await
        .expect("delegated network operation must not panic")
}

pub async fn terrane_platform_tcp_connect_async(
    address: String,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let cancellation = cancellation.clone();
    terrane_platform_network_blocking(move || {
        terrane_platform_tcp_connect(address, deadline, &cancellation)
    })
    .await
}

pub async fn terrane_platform_tcp_connect_host_async(
    host: String,
    port: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let cancellation = cancellation.clone();
    terrane_platform_network_blocking(move || {
        terrane_platform_tcp_connect_host(host, port, deadline, &cancellation)
    })
    .await
}

pub async fn terrane_platform_tcp_accept_async(
    listener: &TerranePlatformCapability,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let listener = listener.clone();
    let cancellation = cancellation.clone();
    terrane_platform_network_blocking(move || {
        terrane_platform_tcp_accept(&listener, deadline, &cancellation)
    })
    .await
}

pub async fn terrane_platform_tcp_read_async(
    stream: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let stream = stream.clone();
    let cancellation = cancellation.clone();
    terrane_platform_network_blocking(move || {
        terrane_platform_tcp_read(&stream, limit, deadline, &cancellation)
    })
    .await
}

pub async fn terrane_platform_tcp_write_async(
    stream: &TerranePlatformCapability,
    data: Vec<u8>,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let stream = stream.clone();
    let cancellation = cancellation.clone();
    terrane_platform_network_blocking(move || {
        terrane_platform_tcp_write(&stream, data, deadline, &cancellation)
    })
    .await
}

pub async fn terrane_platform_udp_send_to_async(
    socket: &TerranePlatformCapability,
    data: Vec<u8>,
    address: String,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let socket = socket.clone();
    let cancellation = cancellation.clone();
    terrane_platform_network_blocking(move || {
        terrane_platform_udp_send_to(&socket, data, address, deadline, &cancellation)
    })
    .await
}

pub async fn terrane_platform_udp_receive_from_async(
    socket: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let socket = socket.clone();
    let cancellation = cancellation.clone();
    terrane_platform_network_blocking(move || {
        terrane_platform_udp_receive_from(&socket, limit, deadline, &cancellation)
    })
    .await
}

pub async fn terrane_platform_dns_lookup_async(
    host: String,
    port: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let cancellation = cancellation.clone();
    terrane_platform_network_blocking(move || {
        terrane_platform_dns_lookup(host, port, deadline, &cancellation)
    })
    .await
}
