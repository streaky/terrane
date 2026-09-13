// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_networking.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support
// Source: case.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let loopback: IpResult = ip_address_from_string(String::from("127.0.0.1"));
        let bind_address: SocketResult = socket_address_from_ip(
            loopback.value,
            terrane_int_support::Int::from(0_i128),
        );
        let bound: ListenerResult = bind_tcp(bind_address.value);
        let listener: TcpListener = bound.value;
        let deadline_options: NetworkOperationOptions = NetworkOperationOptions::terrane_construct(
            terrane_int_support::Int::from(1_i128),
            NetworkCancellationToken::terrane_construct(),
        );
        let expired: StreamResult = __terrane_await((&listener).accept(deadline_options))
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&expired.failed));
        println!("{}", terrane_scalar_support::scalar_text(&expired.deadline_exceeded));
        let cancellation: NetworkCancellationToken = NetworkCancellationToken::terrane_construct();
        network_cancel_operation(cancellation.clone());
        let cancelled_options: NetworkOperationOptions = NetworkOperationOptions::terrane_construct(
            terrane_int_support::Int::from(1000_i128),
            cancellation.clone(),
        );
        let cancelled: StreamResult = __terrane_await(
                (&listener).accept(cancelled_options),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&cancelled.failed));
        println!("{}", terrane_scalar_support::scalar_text(&cancelled.message));
        listener.close();
    });
}
// Source: core/networking.trn
// Namespace: core/networking
#[derive(Clone)]
pub struct NetworkOperationResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl NetworkOperationResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        };
        value.construct(failed, deadline_exceeded, message);
        value
    }
    pub fn construct(&mut self, failed: bool, deadline_exceeded: bool, message: String) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct NetworkCancellationToken {
    pub handle: TerranePlatformCapability,
}
impl NetworkCancellationToken {
    pub fn terrane_construct() -> Self {
        Self {
            handle: terrane_platform_cancellation_token(),
        }
    }
}
pub fn network_cancel_operation(
    cancellation: NetworkCancellationToken,
) -> NetworkOperationResult {
    let raw: TerranePlatformResult = terrane_platform_cancel(&cancellation.handle);
    return NetworkOperationResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
    );
}
#[derive(Clone)]
pub struct NetworkOperationOptions {
    pub deadline_ms: terrane_int_support::Int,
    pub cancellation: NetworkCancellationToken,
}
impl NetworkOperationOptions {
    pub fn terrane_construct(
        deadline_ms: terrane_int_support::Int,
        cancellation: NetworkCancellationToken,
    ) -> Self {
        let mut value = Self {
            deadline_ms: terrane_int_support::Int::from(30000_i128),
            cancellation: NetworkCancellationToken::terrane_construct(),
        };
        value.construct(deadline_ms, cancellation);
        value
    }
    pub fn construct(
        &mut self,
        deadline_ms: terrane_int_support::Int,
        cancellation: NetworkCancellationToken,
    ) {
        self.deadline_ms = deadline_ms.clone();
        self.cancellation = cancellation.clone();
    }
}
pub fn operation_cancellation(
    options: NetworkOperationOptions,
) -> TerranePlatformCapability {
    return options.cancellation.handle;
}
#[derive(Clone)]
pub struct TcpOptions {
    pub no_delay: bool,
    pub ttl: terrane_int_support::Int,
}
impl TcpOptions {
    pub fn terrane_construct(no_delay: bool, ttl: terrane_int_support::Int) -> Self {
        let mut value = Self {
            no_delay: true,
            ttl: terrane_int_support::Int::from(64_i128),
        };
        value.construct(no_delay, ttl);
        value
    }
    pub fn construct(&mut self, no_delay: bool, ttl: terrane_int_support::Int) {
        self.no_delay = no_delay;
        self.ttl = ttl.clone();
    }
}
#[derive(Clone)]
pub struct UdpOptions {
    pub broadcast: bool,
    pub ttl: terrane_int_support::Int,
}
impl UdpOptions {
    pub fn terrane_construct(broadcast: bool, ttl: terrane_int_support::Int) -> Self {
        let mut value = Self {
            broadcast: false,
            ttl: terrane_int_support::Int::from(64_i128),
        };
        value.construct(broadcast, ttl);
        value
    }
    pub fn construct(&mut self, broadcast: bool, ttl: terrane_int_support::Int) {
        self.broadcast = broadcast;
        self.ttl = ttl.clone();
    }
}
#[derive(Clone)]
pub struct IpAddress {
    pub value: String,
    pub version: String,
    pub is_loopback: bool,
}
impl IpAddress {
    pub fn terrane_construct(raw: TerranePlatformResult) -> Self {
        let mut value = Self {
            value: String::from(""),
            version: String::from(""),
            is_loopback: false,
        };
        value.construct(raw);
        value
    }
    pub fn construct(&mut self, raw: TerranePlatformResult) {
        self.value = terrane_platform_result_text(&raw);
        self.version = terrane_platform_result_detail(&raw);
        self.is_loopback = terrane_platform_result_bool(&raw);
    }
    pub fn string(&self) -> String {
        return self.value.clone();
    }
}
#[derive(Clone)]
pub struct IpResult {
    pub failed: bool,
    pub message: String,
    pub value: IpAddress,
}
impl IpResult {
    pub fn terrane_construct(failed: bool, message: String, address: IpAddress) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: IpAddress::terrane_construct(terrane_platform_failed_result()),
        };
        value.construct(failed, message, address);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, address: IpAddress) {
        self.failed = failed;
        self.message = message;
        self.value = address.clone();
    }
}
pub fn ip_address_from_string(text: String) -> IpResult {
    let raw: TerranePlatformResult = terrane_platform_parse_ip(text);
    let failed: bool = terrane_platform_result_failed(&raw);
    let message: String = terrane_platform_result_message(&raw);
    return IpResult::terrane_construct(
        failed,
        message,
        IpAddress::terrane_construct(raw),
    );
}
#[derive(Clone)]
pub struct SocketAddress {
    pub value: String,
    pub ip: IpAddress,
    pub port: terrane_int_support::Int,
}
impl SocketAddress {
    pub fn terrane_construct(
        raw: TerranePlatformResult,
        address_ip: IpAddress,
        address_port: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            value: String::from(""),
            ip: IpAddress::terrane_construct(terrane_platform_failed_result()),
            port: terrane_int_support::Int::from(0_i128),
        };
        value.construct(raw, address_ip, address_port);
        value
    }
    pub fn construct(
        &mut self,
        raw: TerranePlatformResult,
        address_ip: IpAddress,
        address_port: terrane_int_support::Int,
    ) {
        self.value = terrane_platform_result_text(&raw);
        self.ip = address_ip.clone();
        self.port = address_port.clone();
    }
    pub fn string(&self) -> String {
        return self.value.clone();
    }
}
#[derive(Clone)]
pub struct SocketResult {
    pub failed: bool,
    pub message: String,
    pub value: SocketAddress,
}
impl SocketResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        address: SocketAddress,
    ) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: SocketAddress::terrane_construct(
                terrane_platform_failed_result(),
                IpAddress::terrane_construct(terrane_platform_failed_result()),
                terrane_int_support::Int::from(0_i128),
            ),
        };
        value.construct(failed, message, address);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, address: SocketAddress) {
        self.failed = failed;
        self.message = message;
        self.value = address.clone();
    }
}
pub fn socket_address_from_ip(
    ip: IpAddress,
    port: terrane_int_support::Int,
) -> SocketResult {
    let raw: TerranePlatformResult = terrane_platform_parse_socket(&ip.value, &port);
    let failed: bool = terrane_platform_result_failed(&raw);
    let message: String = terrane_platform_result_message(&raw);
    let address: SocketAddress = SocketAddress::terrane_construct(
        raw,
        ip.clone(),
        port.clone(),
    );
    return SocketResult::terrane_construct(failed, message, address);
}
pub fn socket_address_from_string(text: String) -> SocketResult {
    let raw: TerranePlatformResult = terrane_platform_parse_socket_text(text);
    let failed: bool = terrane_platform_result_failed(&raw);
    let message: String = terrane_platform_result_message(&raw);
    let address_ip: IpAddress = IpAddress::terrane_construct(
        terrane_platform_parse_ip(terrane_platform_result_detail(&raw)),
    );
    let port: terrane_int_support::Int = terrane_platform_result_int(&raw);
    let address: SocketAddress = SocketAddress::terrane_construct(
        raw,
        address_ip,
        port.clone(),
    );
    return SocketResult::terrane_construct(failed, message, address);
}
#[derive(Clone)]
pub struct IoResult {
    pub failed: bool,
    pub truncated: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub peer: String,
    pub end: bool,
}
impl IoResult {
    pub fn terrane_construct(
        failed: bool,
        truncated: bool,
        deadline_exceeded: bool,
        message: String,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        peer: String,
        end: bool,
    ) -> Self {
        let mut value = Self {
            failed: false,
            truncated: false,
            deadline_exceeded: false,
            message: String::from(""),
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            peer: String::from(""),
            end: false,
        };
        value
            .construct(
                failed,
                truncated,
                deadline_exceeded,
                message,
                data,
                completed,
                peer,
                end,
            );
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        truncated: bool,
        deadline_exceeded: bool,
        message: String,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        peer: String,
        end: bool,
    ) {
        self.failed = failed;
        self.truncated = truncated;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.data = data;
        self.completed = completed.clone();
        self.peer = peer;
        self.end = end;
    }
}
pub struct TcpStream {
    pub handle: TerranePlatformCapability,
}
impl TcpStream {
    pub fn terrane_construct(resource: TerranePlatformCapability) -> Self {
        let mut value = Self { handle: Default::default() };
        value.construct(resource);
        value
    }
    pub fn construct(&mut self, resource: TerranePlatformCapability) {
        self.handle = resource;
    }
    pub async fn read(
        &self,
        limit: terrane_int_support::Int,
        options: NetworkOperationOptions,
    ) -> IoResult {
        let raw: TerranePlatformResult = __terrane_await(
                terrane_platform_tcp_read_async(
                    &self.handle,
                    limit,
                    options.deadline_ms,
                    &options.cancellation.handle,
                ),
            )
            .await;
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            false,
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
            terrane_platform_result_int(&raw),
            String::from(""),
            terrane_platform_result_bool(&raw),
        );
    }
    pub async fn write(
        &self,
        data: Vec<u8>,
        options: NetworkOperationOptions,
    ) -> IoResult {
        let raw: TerranePlatformResult = __terrane_await(
                terrane_platform_tcp_write_async(
                    &self.handle,
                    data,
                    options.deadline_ms,
                    &options.cancellation.handle,
                ),
            )
            .await;
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            false,
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
            Vec::from([]),
            terrane_platform_result_int(&raw),
            String::from(""),
            false,
        );
    }
    pub fn configure(&self, options: TcpOptions) -> NetworkOperationResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_configure(
            &self.handle,
            options.no_delay,
            options.ttl,
        );
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn shutdown(&self, direction: String) -> NetworkOperationResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_shutdown(
            &self.handle,
            direction,
        );
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn close(self) -> NetworkOperationResult {
        let raw: TerranePlatformResult = terrane_platform_capability_close(&self.handle);
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_capability_close(&self.handle);
    }
}
impl Drop for TcpStream {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct StreamResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub peer: String,
    pub value: TcpStream,
}
impl StreamResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        peer: String,
        stream: TcpStream,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            peer: String::from(""),
            value: TcpStream::terrane_construct(terrane_platform_no_resource()),
        };
        value.construct(failed, deadline_exceeded, message, peer, stream);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        peer: String,
        stream: TcpStream,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.peer = peer;
        self.value = stream;
    }
}
pub async fn connect_tcp(
    address: SocketAddress,
    options: NetworkOperationOptions,
) -> StreamResult {
    let raw: TerranePlatformResult = __terrane_await(
            terrane_platform_tcp_connect_async(
                address.value,
                options.deadline_ms,
                &options.cancellation.handle,
            ),
        )
        .await;
    let stream: TcpStream = TcpStream::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    return StreamResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
        String::from(""),
        stream,
    );
}
pub async fn connect_host(
    host: NetworkHostName,
    port: terrane_int_support::Int,
    options: NetworkOperationOptions,
) -> StreamResult {
    let raw: TerranePlatformResult = __terrane_await(
            terrane_platform_tcp_connect_host_async(
                host.value,
                port,
                options.deadline_ms,
                &options.cancellation.handle,
            ),
        )
        .await;
    let stream: TcpStream = TcpStream::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    return StreamResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_text(&raw),
        stream,
    );
}
pub struct TcpListener {
    pub handle: TerranePlatformCapability,
    pub local_address: String,
}
impl TcpListener {
    pub fn terrane_construct(resource: TerranePlatformCapability) -> Self {
        let mut value = Self {
            handle: Default::default(),
            local_address: String::from(""),
        };
        value.construct(resource);
        value
    }
    pub fn construct(&mut self, resource: TerranePlatformCapability) {
        self.handle = resource;
    }
    pub async fn accept(&self, options: NetworkOperationOptions) -> StreamResult {
        let raw: TerranePlatformResult = __terrane_await(
                terrane_platform_tcp_accept_async(
                    &self.handle,
                    options.deadline_ms,
                    &options.cancellation.handle,
                ),
            )
            .await;
        let stream: TcpStream = TcpStream::terrane_construct(
            terrane_platform_result_capability(&raw),
        );
        return StreamResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_text(&raw),
            stream,
        );
    }
    pub fn close(self) -> NetworkOperationResult {
        let raw: TerranePlatformResult = terrane_platform_capability_close(&self.handle);
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_capability_close(&self.handle);
    }
}
impl Drop for TcpListener {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct ListenerResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub value: TcpListener,
}
impl ListenerResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        listener: TcpListener,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            value: TcpListener::terrane_construct(terrane_platform_no_resource()),
        };
        value.construct(failed, deadline_exceeded, message, listener);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        listener: TcpListener,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.value = listener;
    }
}
pub fn bind_tcp(address: SocketAddress) -> ListenerResult {
    let raw: TerranePlatformResult = terrane_platform_tcp_bind(address.value);
    let mut listener: TcpListener = TcpListener::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    if !terrane_platform_result_failed(&raw) {
        listener.local_address = terrane_platform_result_text(&raw);
    }
    return ListenerResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
        listener,
    );
}
pub struct UdpSocket {
    pub handle: TerranePlatformCapability,
    pub local_address: String,
}
impl UdpSocket {
    pub fn terrane_construct(resource: TerranePlatformCapability) -> Self {
        let mut value = Self {
            handle: Default::default(),
            local_address: String::from(""),
        };
        value.construct(resource);
        value
    }
    pub fn construct(&mut self, resource: TerranePlatformCapability) {
        self.handle = resource;
    }
    pub async fn send_to(
        &self,
        data: Vec<u8>,
        address: SocketAddress,
        options: NetworkOperationOptions,
    ) -> IoResult {
        let raw: TerranePlatformResult = __terrane_await(
                terrane_platform_udp_send_to_async(
                    &self.handle,
                    data,
                    address.value,
                    options.deadline_ms,
                    &options.cancellation.handle,
                ),
            )
            .await;
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            false,
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
            Vec::from([]),
            terrane_platform_result_int(&raw),
            String::from(""),
            false,
        );
    }
    pub async fn receive_from(
        &self,
        limit: terrane_int_support::Int,
        options: NetworkOperationOptions,
    ) -> IoResult {
        let raw: TerranePlatformResult = __terrane_await(
                terrane_platform_udp_receive_from_async(
                    &self.handle,
                    limit,
                    options.deadline_ms,
                    &options.cancellation.handle,
                ),
            )
            .await;
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_truncated(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
            terrane_platform_result_int(&raw),
            terrane_platform_result_text(&raw),
            false,
        );
    }
    pub fn configure(&self, options: UdpOptions) -> NetworkOperationResult {
        let raw: TerranePlatformResult = terrane_platform_udp_configure(
            &self.handle,
            options.broadcast,
            options.ttl,
        );
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn close(self) -> NetworkOperationResult {
        let raw: TerranePlatformResult = terrane_platform_capability_close(&self.handle);
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_capability_close(&self.handle);
    }
}
impl Drop for UdpSocket {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct UdpResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub value: UdpSocket,
}
impl UdpResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        socket: UdpSocket,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            value: UdpSocket::terrane_construct(terrane_platform_no_resource()),
        };
        value.construct(failed, deadline_exceeded, message, socket);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        socket: UdpSocket,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.value = socket;
    }
}
pub fn bind_udp(address: SocketAddress) -> UdpResult {
    let raw: TerranePlatformResult = terrane_platform_udp_bind(address.value);
    let mut socket: UdpSocket = UdpSocket::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    if !terrane_platform_result_failed(&raw) {
        socket.local_address = terrane_platform_result_text(&raw);
    }
    return UdpResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
        socket,
    );
}
#[derive(Clone)]
pub struct DnsResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub candidates: terrane_collection_support::List<String>,
    pub ttl: terrane_int_support::Int,
    pub ttl_known: bool,
}
impl DnsResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        ttl: terrane_int_support::Int,
        ttl_known: bool,
        candidates: terrane_collection_support::List<String>,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            candidates: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            ttl: terrane_int_support::Int::from(0_i128),
            ttl_known: false,
        };
        value.construct(failed, deadline_exceeded, message, ttl, ttl_known, candidates);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        ttl: terrane_int_support::Int,
        ttl_known: bool,
        candidates: terrane_collection_support::List<String>,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.ttl = ttl.clone();
        self.ttl_known = ttl_known;
        self.candidates = candidates.clone();
    }
}
#[derive(Clone)]
pub struct NetworkHostName {
    pub value: String,
}
impl NetworkHostName {
    pub fn terrane_construct(raw: TerranePlatformResult) -> Self {
        let mut value = Self { value: String::from("") };
        value.construct(raw);
        value
    }
    pub fn construct(&mut self, raw: TerranePlatformResult) {
        self.value = terrane_platform_result_text(&raw);
    }
}
#[derive(Clone)]
pub struct NetworkHostNameResult {
    pub failed: bool,
    pub message: String,
    pub value: NetworkHostName,
}
impl NetworkHostNameResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        host: NetworkHostName,
    ) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: NetworkHostName::terrane_construct(terrane_platform_failed_result()),
        };
        value.construct(failed, message, host);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, host: NetworkHostName) {
        self.failed = failed;
        self.message = message;
        self.value = host.clone();
    }
}
pub fn parse_host_name(text: String) -> NetworkHostNameResult {
    let raw: TerranePlatformResult = terrane_platform_parse_host_name(text);
    let failed: bool = terrane_platform_result_failed(&raw);
    let message: String = terrane_platform_result_message(&raw);
    let host: NetworkHostName = NetworkHostName::terrane_construct(raw);
    return NetworkHostNameResult::terrane_construct(failed, message, host);
}
pub async fn lookup_dns(
    host: NetworkHostName,
    port: terrane_int_support::Int,
    options: NetworkOperationOptions,
) -> DnsResult {
    let raw: TerranePlatformResult = __terrane_await(
            terrane_platform_dns_lookup_async(
                host.value,
                port,
                options.deadline_ms,
                &options.cancellation.handle,
            ),
        )
        .await;
    let raw_candidates: Vec<String> = terrane_platform_result_entries(&raw);
    let mut candidates: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("")]);
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = candidates.make_unique();
        while index.clone()
            < terrane_int_support::Int::from(raw_candidates.len() as i128)
        {
            __terrane_list_append_0
                .push(
                    __terrane_raised(
                        raw_candidates
                            .get(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(&index.clone()),
                                    0 /* terrane-site: core/networking.trn:319:28-319:49 */,
                                ),
                            )
                            .cloned()
                            .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(&index.clone()),
                                    0 /* terrane-site: core/networking.trn:319:28-319:49 */,
                                ),
                            )),
                        0 /* terrane-site: core/networking.trn:319:28-319:49 */,
                    ),
                );
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return DnsResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_int(&raw),
        terrane_platform_result_bool(&raw),
        candidates.clone(),
    );
}
