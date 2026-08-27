// Generated deterministically by Terrane <version>.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TerraneErrorKind {
    ArithmeticOverflow,
    DivisionByZero,
    IntegerConversionOverflow,
    NegativeShiftCount,
    CoercionError,
    DecodeError,
    IndexError,
    MissingKey,
    ResourceError,
    SourceError,
}
impl TerraneErrorKind {
    fn from_source_name(name: &str) -> Self {
        match name {
            ".arithmetic-overflow" => Self::ArithmeticOverflow,
            ".division-by-zero" => Self::DivisionByZero,
            ".integer-conversion-overflow" => Self::IntegerConversionOverflow,
            ".negative-shift-count" => Self::NegativeShiftCount,
            ".coercion-error" => Self::CoercionError,
            ".decode-error" => Self::DecodeError,
            ".index-error" => Self::IndexError,
            ".missing-key" => Self::MissingKey,
            ".resource-error" => Self::ResourceError,
            _ => Self::SourceError,
        }
    }
    fn source_name(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => ".arithmetic-overflow",
            Self::DivisionByZero => ".division-by-zero",
            Self::IntegerConversionOverflow => ".integer-conversion-overflow",
            Self::NegativeShiftCount => ".negative-shift-count",
            Self::CoercionError => ".coercion-error",
            Self::DecodeError => ".decode-error",
            Self::IndexError => ".index-error",
            Self::MissingKey => ".missing-key",
            Self::ResourceError => ".resource-error",
            Self::SourceError => ".error",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    message: String,
    cause: Option<Box<TerraneError>>,
    context: Vec<&'static str>,
}
impl TerraneError {
    fn new(kind: TerraneErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            cause: None,
            context: Vec::new(),
        }
    }
    #[allow(dead_code)]
    fn at(mut self, frame: &'static str) -> Self {
        self.context.push(frame);
        self
    }
    fn render(&self) -> String {
        let mut rendered = format!("{}: {}", self.kind.source_name(), self.message);
        if let Some(cause) = &self.cause {
            rendered.push_str("\ncaused by: ");
            rendered.push_str(&cause.render());
        }
        for frame in &self.context {
            rendered.push_str("\nat ");
            rendered.push_str(frame);
        }
        rendered
    }
}
impl std::fmt::Display for TerraneError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.render())
    }
}
impl From<terrane_int_support::ArithmeticError> for TerraneError {
    fn from(error: terrane_int_support::ArithmeticError) -> Self {
        Self::new(
            TerraneErrorKind::from_source_name(error.source_name()),
            error.to_string(),
        )
    }
}
impl From<terrane_string_support::DecodeError> for TerraneError {
    fn from(error: terrane_string_support::DecodeError) -> Self {
        Self::new(
            TerraneErrorKind::DecodeError,
            error.to_string().trim_start_matches(".decode-error: "),
        )
    }
}
impl From<terrane_collection_support::IndexError> for TerraneError {
    fn from(error: terrane_collection_support::IndexError) -> Self {
        Self::new(TerraneErrorKind::IndexError, error.to_string())
    }
}
impl From<terrane_collection_support::MissingKey> for TerraneError {
    fn from(error: terrane_collection_support::MissingKey) -> Self {
        Self::new(TerraneErrorKind::MissingKey, error.to_string())
    }
}
impl From<terrane_collection_support::RangeStepError> for TerraneError {
    fn from(error: terrane_collection_support::RangeStepError) -> Self {
        Self::new(TerraneErrorKind::SourceError, error.to_string())
    }
}
fn __terrane_uncaught(error: TerraneError) -> ! {
    eprintln!("{}", error.render());
    std::process::exit(1);
}
fn __terrane_generated_defect(message: &str) -> ! {
    eprintln!(
        "internal compiler defect: generated program reached an impossible completion: {message}"
    );
    std::process::exit(5);
}
#[allow(dead_code)]
enum TerraneCompletion<T> {
    Normal,
    Return(T),
    Error(TerraneError),
    Break,
    Continue,
}
type TerranePlatformCapability = terrane_platform_support::Capability;
type TerranePlatformResult = terrane_platform_support::ResultValue;
fn terrane_platform_i128(
    value: &terrane_int_support::Int,
    label: &str,
) -> Result<i128, TerranePlatformResult> {
    terrane_int_support::coerce::<i128>(value)
        .map_err(|_| TerranePlatformResult::error(
            format!("{label} is outside the signed 128-bit platform range"),
        ))
}
macro_rules! terrane_platform_i128 {
    ($value:expr, $label:literal) => {
        match terrane_platform_i128(&$value, $label) { Ok(value) => value, Err(error) =>
        return error, }
    };
}
#[allow(dead_code)]
fn terrane_platform_cancellation_token() -> TerranePlatformCapability {
    terrane_platform_support::cancellation_token()
}
#[allow(dead_code)]
fn terrane_platform_no_resource() -> TerranePlatformCapability {
    TerranePlatformCapability::default()
}
#[allow(dead_code)]
fn terrane_platform_failed_result() -> TerranePlatformResult {
    TerranePlatformResult::error("uninitialized platform value")
}
#[allow(dead_code)]
fn terrane_platform_cancel(token: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::cancel(token)
}
#[allow(dead_code)]
fn terrane_platform_result_failed(result: &TerranePlatformResult) -> bool {
    result.failed
}
#[allow(dead_code)]
fn terrane_platform_result_resource_limit(result: &TerranePlatformResult) -> bool {
    result.resource_limit
}
#[allow(dead_code)]
fn terrane_platform_result_truncated(result: &TerranePlatformResult) -> bool {
    result.truncated
}
#[allow(dead_code)]
fn terrane_platform_result_deadline_exceeded(result: &TerranePlatformResult) -> bool {
    result.deadline_exceeded
}
#[allow(dead_code)]
fn terrane_platform_result_message(result: &TerranePlatformResult) -> String {
    result.message.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_text(result: &TerranePlatformResult) -> String {
    result.text.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_detail(result: &TerranePlatformResult) -> String {
    result.detail.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_bytes(result: &TerranePlatformResult) -> Vec<u8> {
    result.data.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_int(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
#[allow(dead_code)]
fn terrane_platform_result_bool(result: &TerranePlatformResult) -> bool {
    result.flag
}
#[allow(dead_code)]
fn terrane_platform_result_entries(result: &TerranePlatformResult) -> Vec<String> {
    result.entries.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_capability(
    result: &TerranePlatformResult,
) -> TerranePlatformCapability {
    result.capability.clone().unwrap_or_default()
}
fn terrane_platform_parse_ip(text: String) -> TerranePlatformResult {
    terrane_platform_support::parse_ip(&text)
}
fn terrane_platform_parse_host_name(text: String) -> TerranePlatformResult {
    terrane_platform_support::parse_host_name(&text)
}
fn terrane_platform_parse_socket(
    ip: &String,
    port: &terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::parse_socket(
        ip,
        terrane_platform_i128!(port, "socket port"),
    )
}
fn terrane_platform_tcp_bind(address: String) -> TerranePlatformResult {
    terrane_platform_support::tcp_bind(&address)
}
fn terrane_platform_tcp_connect(
    address: String,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_connect(
        &address,
        terrane_platform_i128!(deadline, "TCP connect deadline"),
        cancellation,
    )
}
fn terrane_platform_tcp_connect_host(
    host: String,
    port: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_connect_host(
        &host,
        terrane_platform_i128!(port, "TCP host port"),
        terrane_platform_i128!(deadline, "TCP host connect deadline"),
        cancellation,
    )
}
fn terrane_platform_tcp_accept(
    listener: &TerranePlatformCapability,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_accept(
        listener,
        terrane_platform_i128!(deadline, "TCP accept deadline"),
        cancellation,
    )
}
fn terrane_platform_tcp_read(
    stream: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_read(
        stream,
        terrane_platform_i128!(limit, "TCP read limit"),
        terrane_platform_i128!(deadline, "TCP read deadline"),
        cancellation,
    )
}
fn terrane_platform_tcp_write(
    stream: &TerranePlatformCapability,
    data: Vec<u8>,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_write(
        stream,
        &data,
        terrane_platform_i128!(deadline, "TCP write deadline"),
        cancellation,
    )
}
fn terrane_platform_tcp_shutdown(
    stream: &TerranePlatformCapability,
    direction: String,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_shutdown(stream, &direction)
}
fn terrane_platform_tcp_configure(
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
fn terrane_platform_udp_bind(address: String) -> TerranePlatformResult {
    terrane_platform_support::udp_bind(&address)
}
fn terrane_platform_udp_configure(
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
fn terrane_platform_udp_send_to(
    socket: &TerranePlatformCapability,
    data: Vec<u8>,
    address: String,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::udp_send_to(
        socket,
        &data,
        &address,
        terrane_platform_i128!(deadline, "UDP send deadline"),
        cancellation,
    )
}
fn terrane_platform_udp_receive_from(
    socket: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::udp_receive_from(
        socket,
        terrane_platform_i128!(limit, "UDP receive limit"),
        terrane_platform_i128!(deadline, "UDP receive deadline"),
        cancellation,
    )
}
fn terrane_platform_dns_lookup(
    host: String,
    port: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::dns_lookup(
        &host,
        terrane_platform_i128!(port, "DNS port"),
        terrane_platform_i128!(deadline, "DNS deadline"),
        cancellation,
    )
}
fn terrane_platform_close(
    capability: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::close(capability)
}
fn terrane_platform_tls_client(
    stream: &TerranePlatformCapability,
    server: String,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tls_client(
        stream,
        &server,
        terrane_platform_i128!(deadline, "TLS deadline"),
        cancellation,
    )
}
fn terrane_platform_tls_read(
    stream: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tls_read(
        stream,
        terrane_platform_i128!(limit, "TLS read limit"),
        terrane_platform_i128!(deadline, "TLS read deadline"),
        cancellation,
    )
}
fn terrane_platform_tls_write(
    stream: &TerranePlatformCapability,
    data: Vec<u8>,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tls_write(
        stream,
        &data,
        terrane_platform_i128!(deadline, "TLS write deadline"),
        cancellation,
    )
}
fn terrane_platform_tls_shutdown(
    stream: &TerranePlatformCapability,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tls_shutdown(
        stream,
        terrane_platform_i128!(deadline, "TLS shutdown deadline"),
        cancellation,
    )
}
// Source: case.trn
// Namespace: app
fn main() {
    return ();
}
// Source: standard/tls.trn
// Namespace: standard/tls
pub struct TlsStream {
    pub handle: TerranePlatformCapability,
    pub negotiated_version: String,
}
impl TlsStream {
    pub fn terrane_construct(resource: TerranePlatformCapability) -> Self {
        let mut value = Self {
            handle: Default::default(),
            negotiated_version: String::from(""),
        };
        value.construct(resource);
        value
    }
    pub fn construct(&mut self, resource: TerranePlatformCapability) {
        self.handle = resource;
    }
    pub fn read(
        &self,
        limit: terrane_int_support::Int,
        options: OperationOptions,
    ) -> IoResult {
        let cancellation: TerranePlatformCapability = operation_cancellation(
            options.clone(),
        );
        let raw: TerranePlatformResult = terrane_platform_tls_read(
            &self.handle,
            limit,
            options.deadline_ms,
            &cancellation,
        );
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
    pub fn write(&self, data: Vec<u8>, options: OperationOptions) -> IoResult {
        let cancellation: TerranePlatformCapability = operation_cancellation(
            options.clone(),
        );
        let raw: TerranePlatformResult = terrane_platform_tls_write(
            &self.handle,
            data,
            options.deadline_ms,
            &cancellation,
        );
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
    pub fn close(&self) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_close(&self.handle);
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn shutdown(&self, options: OperationOptions) -> OperationResult {
        let cancellation: TerranePlatformCapability = operation_cancellation(
            options.clone(),
        );
        let raw: TerranePlatformResult = terrane_platform_tls_shutdown(
            &self.handle,
            options.deadline_ms,
            &cancellation,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn destruct(&self) {
        terrane_platform_close(&self.handle);
    }
}
impl Drop for TlsStream {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct TlsResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub value: TlsStream,
}
impl TlsResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        stream: TlsStream,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            value: TlsStream::terrane_construct(terrane_platform_no_resource()),
        };
        value.construct(failed, deadline_exceeded, message, stream);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        stream: TlsStream,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.value = stream;
    }
}
pub fn connect_tls(
    stream: TcpStream,
    server_name: HostName,
    options: OperationOptions,
) -> TlsResult {
    let cancellation: TerranePlatformCapability = operation_cancellation(
        options.clone(),
    );
    let raw: TerranePlatformResult = terrane_platform_tls_client(
        &stream.handle,
        server_name.value,
        options.deadline_ms,
        &cancellation,
    );
    let mut value: TlsStream = TlsStream::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    if !terrane_platform_result_failed(&raw) {
        value.negotiated_version = terrane_platform_result_text(&raw);
    }
    return TlsResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
        value,
    );
}
// Source: standard/networking.trn
// Namespace: standard/networking
#[derive(Clone)]
pub struct OperationResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl OperationResult {
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
pub struct CancellationToken {
    pub handle: TerranePlatformCapability,
}
impl CancellationToken {
    pub fn terrane_construct() -> Self {
        Self {
            handle: terrane_platform_cancellation_token(),
        }
    }
}
pub fn cancel_operation(cancellation: CancellationToken) -> OperationResult {
    let raw: TerranePlatformResult = terrane_platform_cancel(&cancellation.handle);
    return OperationResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
    );
}
#[derive(Clone)]
pub struct OperationOptions {
    pub deadline_ms: terrane_int_support::Int,
    pub cancellation: CancellationToken,
}
impl OperationOptions {
    pub fn terrane_construct(
        deadline_ms: terrane_int_support::Int,
        cancellation: CancellationToken,
    ) -> Self {
        let mut value = Self {
            deadline_ms: terrane_int_support::Int::from(30000_i128),
            cancellation: CancellationToken::terrane_construct(),
        };
        value.construct(deadline_ms, cancellation);
        value
    }
    pub fn construct(
        &mut self,
        deadline_ms: terrane_int_support::Int,
        cancellation: CancellationToken,
    ) {
        self.deadline_ms = deadline_ms.clone();
        self.cancellation = cancellation.clone();
    }
}
pub fn operation_cancellation(options: OperationOptions) -> TerranePlatformCapability {
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
    return SocketResult::terrane_construct(failed, message, address.clone());
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
    pub fn read(
        &self,
        limit: terrane_int_support::Int,
        options: OperationOptions,
    ) -> IoResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_read(
            &self.handle,
            limit,
            options.deadline_ms,
            &options.cancellation.handle,
        );
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
    pub fn write(&self, data: Vec<u8>, options: OperationOptions) -> IoResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_write(
            &self.handle,
            data,
            options.deadline_ms,
            &options.cancellation.handle,
        );
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
    pub fn configure(&self, options: TcpOptions) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_configure(
            &self.handle,
            options.no_delay,
            options.ttl,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn shutdown(&self, direction: String) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_shutdown(
            &self.handle,
            direction,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn close(&self) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_close(&self.handle);
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn destruct(&self) {
        terrane_platform_close(&self.handle);
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
pub fn connect_tcp(address: SocketAddress, options: OperationOptions) -> StreamResult {
    let raw: TerranePlatformResult = terrane_platform_tcp_connect(
        address.value,
        options.deadline_ms,
        &options.cancellation.handle,
    );
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
pub fn connect_host(
    host: HostName,
    port: terrane_int_support::Int,
    options: OperationOptions,
) -> StreamResult {
    let raw: TerranePlatformResult = terrane_platform_tcp_connect_host(
        host.value,
        port,
        options.deadline_ms,
        &options.cancellation.handle,
    );
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
    pub fn accept(&self, options: OperationOptions) -> StreamResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_accept(
            &self.handle,
            options.deadline_ms,
            &options.cancellation.handle,
        );
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
    pub fn close(&self) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_close(&self.handle);
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn destruct(&self) {
        terrane_platform_close(&self.handle);
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
    pub fn send_to(
        &self,
        data: Vec<u8>,
        address: SocketAddress,
        options: OperationOptions,
    ) -> IoResult {
        let raw: TerranePlatformResult = terrane_platform_udp_send_to(
            &self.handle,
            data,
            address.value,
            options.deadline_ms,
            &options.cancellation.handle,
        );
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
    pub fn receive_from(
        &self,
        limit: terrane_int_support::Int,
        options: OperationOptions,
    ) -> IoResult {
        let raw: TerranePlatformResult = terrane_platform_udp_receive_from(
            &self.handle,
            limit,
            options.deadline_ms,
            &options.cancellation.handle,
        );
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
    pub fn configure(&self, options: UdpOptions) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_udp_configure(
            &self.handle,
            options.broadcast,
            options.ttl,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn close(&self) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_close(&self.handle);
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn destruct(&self) {
        terrane_platform_close(&self.handle);
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
pub struct HostName {
    pub value: String,
}
impl HostName {
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
pub struct HostNameResult {
    pub failed: bool,
    pub message: String,
    pub value: HostName,
}
impl HostNameResult {
    pub fn terrane_construct(failed: bool, message: String, host: HostName) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: HostName::terrane_construct(terrane_platform_failed_result()),
        };
        value.construct(failed, message, host);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, host: HostName) {
        self.failed = failed;
        self.message = message;
        self.value = host.clone();
    }
}
pub fn parse_host_name(text: String) -> HostNameResult {
    let raw: TerranePlatformResult = terrane_platform_parse_host_name(text);
    let failed: bool = terrane_platform_result_failed(&raw);
    let message: String = terrane_platform_result_message(&raw);
    let host: HostName = HostName::terrane_construct(raw);
    return HostNameResult::terrane_construct(failed, message, host.clone());
}
pub fn lookup_dns(
    host: HostName,
    port: terrane_int_support::Int,
    options: OperationOptions,
) -> DnsResult {
    let raw: TerranePlatformResult = terrane_platform_dns_lookup(
        host.value,
        port,
        options.deadline_ms,
        &options.cancellation.handle,
    );
    let raw_candidates: Vec<String> = terrane_platform_result_entries(&raw);
    let mut candidates: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("")]);
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < terrane_int_support::Int::from(raw_candidates.len() as i128) {
        candidates
            .append(
                raw_candidates
                    .get(
                        terrane_collection_support::index_from_int(&index.clone())
                            .unwrap_or_else(|error| __terrane_uncaught(
                                TerraneError::from(error)
                                    .at(
                                        "/standard/networking::lookup-dns (networking.trn:317:28)",
                                    ),
                            )),
                    )
                    .cloned()
                    .ok_or(terrane_collection_support::IndexError {
                        index: terrane_collection_support::index_from_int(&index.clone())
                            .unwrap_or_else(|error| __terrane_uncaught(
                                TerraneError::from(error)
                                    .at(
                                        "/standard/networking::lookup-dns (networking.trn:317:28)",
                                    ),
                            )),
                    })
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/networking::lookup-dns (networking.trn:317:28)",
                            ),
                    )),
            );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
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
