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
    result.capability.clone().expect("successful platform result carries capability")
}
fn terrane_platform_parse_ip(text: String) -> TerranePlatformResult {
    terrane_platform_support::parse_ip(&text)
}
fn terrane_platform_parse_socket(
    ip: &String,
    port: &terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::parse_socket(
        ip,
        port.as_big().to_string().parse::<i128>().unwrap_or(-1),
    )
}
fn terrane_platform_tcp_bind(address: String) -> TerranePlatformResult {
    terrane_platform_support::tcp_bind(&address)
}
fn terrane_platform_tcp_connect(
    address: String,
    deadline: terrane_int_support::Int,
    cancelled: bool,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_connect(
        &address,
        deadline.as_big().to_string().parse::<i128>().unwrap_or(-1),
        cancelled,
    )
}
fn terrane_platform_tcp_accept(
    listener: &TerranePlatformCapability,
    deadline: terrane_int_support::Int,
    cancelled: bool,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_accept(
        listener,
        deadline.as_big().to_string().parse::<i128>().unwrap_or(-1),
        cancelled,
    )
}
fn terrane_platform_tcp_read(
    stream: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancelled: bool,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_read(
        stream,
        limit.as_big().to_string().parse::<i128>().unwrap_or(-1),
        deadline.as_big().to_string().parse::<i128>().unwrap_or(-1),
        cancelled,
    )
}
fn terrane_platform_tcp_write(
    stream: &TerranePlatformCapability,
    data: Vec<u8>,
    deadline: terrane_int_support::Int,
    cancelled: bool,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_write(
        stream,
        &data,
        deadline.as_big().to_string().parse::<i128>().unwrap_or(-1),
        cancelled,
    )
}
fn terrane_platform_tcp_shutdown(
    stream: &TerranePlatformCapability,
    direction: String,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_shutdown(stream, &direction)
}
fn terrane_platform_udp_bind(address: String) -> TerranePlatformResult {
    terrane_platform_support::udp_bind(&address)
}
fn terrane_platform_udp_send_to(
    socket: &TerranePlatformCapability,
    data: Vec<u8>,
    address: String,
    deadline: terrane_int_support::Int,
    cancelled: bool,
) -> TerranePlatformResult {
    terrane_platform_support::udp_send_to(
        socket,
        &data,
        &address,
        deadline.as_big().to_string().parse::<i128>().unwrap_or(-1),
        cancelled,
    )
}
fn terrane_platform_udp_receive_from(
    socket: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancelled: bool,
) -> TerranePlatformResult {
    terrane_platform_support::udp_receive_from(
        socket,
        limit.as_big().to_string().parse::<i128>().unwrap_or(-1),
        deadline.as_big().to_string().parse::<i128>().unwrap_or(-1),
        cancelled,
    )
}
fn terrane_platform_dns_lookup(
    host: String,
    port: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancelled: bool,
) -> TerranePlatformResult {
    terrane_platform_support::dns_lookup(
        &host,
        port.as_big().to_string().parse::<i128>().unwrap_or(-1),
        deadline.as_big().to_string().parse::<i128>().unwrap_or(-1),
        cancelled,
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
    cancelled: bool,
) -> TerranePlatformResult {
    terrane_platform_support::tls_client(
        stream,
        &server,
        deadline.as_big().to_string().parse::<i128>().unwrap_or(-1),
        cancelled,
    )
}
fn terrane_platform_tls_read(
    stream: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancelled: bool,
) -> TerranePlatformResult {
    terrane_platform_support::tls_read(
        stream,
        limit.as_big().to_string().parse::<i128>().unwrap_or(-1),
        deadline.as_big().to_string().parse::<i128>().unwrap_or(-1),
        cancelled,
    )
}
fn terrane_platform_tls_write(
    stream: &TerranePlatformCapability,
    data: Vec<u8>,
    deadline: terrane_int_support::Int,
    cancelled: bool,
) -> TerranePlatformResult {
    terrane_platform_support::tls_write(
        stream,
        &data,
        deadline.as_big().to_string().parse::<i128>().unwrap_or(-1),
        cancelled,
    )
}
// Source: case.trn
// Namespace: app
fn consume(value: TlsStream) {
    let _ = &value;
}
fn main() {
    let stream: TlsStream = TlsStream::terrane_construct();
    consume(stream);
}
// Source: standard/tls.trn
// Namespace: standard/tls
pub struct TlsStream {
    pub handle: TerranePlatformCapability,
}
impl TlsStream {
    pub fn terrane_construct() -> Self {
        Self { handle: Default::default() }
    }
    pub fn read(
        &self,
        limit: terrane_int_support::Int,
        options: OperationOptions,
    ) -> IoResult {
        let raw: TerranePlatformResult = terrane_platform_tls_read(
            &self.handle,
            limit,
            options.deadline_ms,
            options.cancelled,
        );
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            false,
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
            terrane_platform_result_int(&raw),
            String::from(""),
            terrane_platform_result_bool(&raw),
        );
    }
    pub fn write(&self, data: Vec<u8>, options: OperationOptions) -> IoResult {
        let raw: TerranePlatformResult = terrane_platform_tls_write(
            &self.handle,
            data,
            options.deadline_ms,
            options.cancelled,
        );
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            false,
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
    pub message: String,
    pub value: TlsStream,
}
impl TlsResult {
    pub fn terrane_construct(failed: bool, message: String, stream: TlsStream) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: TlsStream::terrane_construct(),
        };
        value.construct(failed, message, stream);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, stream: TlsStream) {
        self.failed = failed;
        self.message = message;
        self.value = stream;
    }
}
pub fn connect_tls(
    stream: TcpStream,
    server_name: HostName,
    options: OperationOptions,
) -> TlsResult {
    let raw: TerranePlatformResult = terrane_platform_tls_client(
        &stream.handle,
        server_name.value,
        options.deadline_ms,
        options.cancelled,
    );
    let mut value: TlsStream = TlsStream::terrane_construct();
    if !terrane_platform_result_failed(&raw) {
        value.handle = terrane_platform_result_capability(&raw);
    }
    return TlsResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        value,
    );
}
// Source: standard/networking.trn
// Namespace: standard/networking
#[derive(Clone)]
pub struct OperationOptions {
    pub deadline_ms: terrane_int_support::Int,
    pub cancelled: bool,
}
impl OperationOptions {
    pub fn terrane_construct(
        deadline_ms: terrane_int_support::Int,
        cancelled: bool,
    ) -> Self {
        let mut value = Self {
            deadline_ms: terrane_int_support::Int::from(30000_i128),
            cancelled: false,
        };
        value.construct(deadline_ms, cancelled);
        value
    }
    pub fn construct(&mut self, deadline_ms: terrane_int_support::Int, cancelled: bool) {
        self.deadline_ms = deadline_ms.clone();
        self.cancelled = cancelled;
    }
}
#[derive(Clone)]
pub struct OperationResult {
    pub failed: bool,
    pub message: String,
}
impl OperationResult {
    pub fn terrane_construct(failed: bool, message: String) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
        };
        value.construct(failed, message);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String) {
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct IpAddress {
    pub value: String,
    pub version: String,
    pub is_loopback: bool,
}
impl IpAddress {
    pub fn terrane_construct(text: String, version: String, is_loopback: bool) -> Self {
        let mut value = Self {
            value: String::from(""),
            version: String::from(""),
            is_loopback: false,
        };
        value.construct(text, version, is_loopback);
        value
    }
    pub fn construct(&mut self, text: String, version: String, is_loopback: bool) {
        self.value = text;
        self.version = version;
        self.is_loopback = is_loopback;
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
            value: IpAddress::terrane_construct(
                String::from(""),
                String::from(""),
                false,
            ),
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
    return IpResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        IpAddress::terrane_construct(
            terrane_platform_result_text(&raw),
            terrane_platform_result_detail(&raw),
            terrane_platform_result_bool(&raw),
        ),
    );
}
#[derive(Clone)]
pub struct SocketAddress {
    pub value: String,
    pub ip: IpAddress,
    pub port: terrane_int_support::Int,
}
impl SocketAddress {
    pub fn terrane_construct(ip: IpAddress, port: terrane_int_support::Int) -> Self {
        let mut value = Self {
            value: String::from(""),
            ip: IpAddress::terrane_construct(String::from(""), String::from(""), false),
            port: terrane_int_support::Int::from(0_i128),
        };
        value.construct(ip, port);
        value
    }
    pub fn construct(&mut self, ip: IpAddress, port: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_parse_socket(&ip.value, &port);
        self.value = terrane_platform_result_text(&raw);
        self.ip = ip.clone();
        self.port = port.clone();
    }
    pub fn string(&self) -> String {
        return self.value.clone();
    }
}
#[derive(Clone)]
pub struct IoResult {
    pub failed: bool,
    pub truncated: bool,
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
        message: String,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        peer: String,
        end: bool,
    ) -> Self {
        let mut value = Self {
            failed: false,
            truncated: false,
            message: String::from(""),
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            peer: String::from(""),
            end: false,
        };
        value.construct(failed, truncated, message, data, completed, peer, end);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        truncated: bool,
        message: String,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        peer: String,
        end: bool,
    ) {
        self.failed = failed;
        self.truncated = truncated;
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
    pub fn terrane_construct() -> Self {
        Self { handle: Default::default() }
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
            options.cancelled,
        );
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            false,
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
            options.cancelled,
        );
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            false,
            terrane_platform_result_message(&raw),
            Vec::from([]),
            terrane_platform_result_int(&raw),
            String::from(""),
            false,
        );
    }
    pub fn shutdown(&self, direction: String) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_shutdown(
            &self.handle,
            direction,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn close(&self) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_close(&self.handle);
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
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
    pub message: String,
    pub peer: String,
    pub value: TcpStream,
}
impl StreamResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        peer: String,
        stream: TcpStream,
    ) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            peer: String::from(""),
            value: TcpStream::terrane_construct(),
        };
        value.construct(failed, message, peer, stream);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        peer: String,
        stream: TcpStream,
    ) {
        self.failed = failed;
        self.message = message;
        self.peer = peer;
        self.value = stream;
    }
}
pub fn connect_tcp(address: SocketAddress, options: OperationOptions) -> StreamResult {
    let raw: TerranePlatformResult = terrane_platform_tcp_connect(
        address.value,
        options.deadline_ms,
        options.cancelled,
    );
    let mut stream: TcpStream = TcpStream::terrane_construct();
    if !terrane_platform_result_failed(&raw) {
        stream.handle = terrane_platform_result_capability(&raw);
    }
    return StreamResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        String::from(""),
        stream,
    );
}
pub struct TcpListener {
    pub handle: TerranePlatformCapability,
    pub local_address: String,
}
impl TcpListener {
    pub fn terrane_construct() -> Self {
        Self {
            handle: Default::default(),
            local_address: String::from(""),
        }
    }
    pub fn accept(&self, options: OperationOptions) -> StreamResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_accept(
            &self.handle,
            options.deadline_ms,
            options.cancelled,
        );
        let mut stream: TcpStream = TcpStream::terrane_construct();
        if !terrane_platform_result_failed(&raw) {
            stream.handle = terrane_platform_result_capability(&raw);
        }
        return StreamResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_text(&raw),
            stream,
        );
    }
    pub fn close(&self) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_close(&self.handle);
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
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
    pub message: String,
    pub value: TcpListener,
}
impl ListenerResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        listener: TcpListener,
    ) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: TcpListener::terrane_construct(),
        };
        value.construct(failed, message, listener);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, listener: TcpListener) {
        self.failed = failed;
        self.message = message;
        self.value = listener;
    }
}
pub fn bind_tcp(address: SocketAddress) -> ListenerResult {
    let raw: TerranePlatformResult = terrane_platform_tcp_bind(address.value);
    let mut listener: TcpListener = TcpListener::terrane_construct();
    if !terrane_platform_result_failed(&raw) {
        listener.handle = terrane_platform_result_capability(&raw);
        listener.local_address = terrane_platform_result_text(&raw);
    }
    return ListenerResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        listener,
    );
}
pub struct UdpSocket {
    pub handle: TerranePlatformCapability,
    pub local_address: String,
}
impl UdpSocket {
    pub fn terrane_construct() -> Self {
        Self {
            handle: Default::default(),
            local_address: String::from(""),
        }
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
            options.cancelled,
        );
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            false,
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
            options.cancelled,
        );
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_truncated(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
            terrane_platform_result_int(&raw),
            terrane_platform_result_text(&raw),
            false,
        );
    }
    pub fn close(&self) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_close(&self.handle);
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
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
    pub message: String,
    pub value: UdpSocket,
}
impl UdpResult {
    pub fn terrane_construct(failed: bool, message: String, socket: UdpSocket) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: UdpSocket::terrane_construct(),
        };
        value.construct(failed, message, socket);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, socket: UdpSocket) {
        self.failed = failed;
        self.message = message;
        self.value = socket;
    }
}
pub fn bind_udp(address: SocketAddress) -> UdpResult {
    let raw: TerranePlatformResult = terrane_platform_udp_bind(address.value);
    let mut socket: UdpSocket = UdpSocket::terrane_construct();
    if !terrane_platform_result_failed(&raw) {
        socket.handle = terrane_platform_result_capability(&raw);
        socket.local_address = terrane_platform_result_text(&raw);
    }
    return UdpResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        socket,
    );
}
#[derive(Clone)]
pub struct DnsResult {
    pub failed: bool,
    pub message: String,
    pub candidates: terrane_collection_support::List<String>,
    pub ttl: terrane_int_support::Int,
    pub ttl_known: bool,
}
impl DnsResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        ttl: terrane_int_support::Int,
        ttl_known: bool,
        candidates: terrane_collection_support::List<String>,
    ) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            candidates: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            ttl: terrane_int_support::Int::from(0_i128),
            ttl_known: false,
        };
        value.construct(failed, message, ttl, ttl_known, candidates);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        ttl: terrane_int_support::Int,
        ttl_known: bool,
        candidates: terrane_collection_support::List<String>,
    ) {
        self.failed = failed;
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
    pub fn terrane_construct(address: String) -> Self {
        let mut value = Self { value: String::from("") };
        value.construct(address);
        value
    }
    pub fn construct(&mut self, address: String) {
        self.value = address;
    }
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
        options.cancelled,
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
                                        "/standard/networking::lookup-dns (networking.trn:227:28)",
                                    ),
                            )),
                    )
                    .cloned()
                    .ok_or(terrane_collection_support::IndexError {
                        index: terrane_collection_support::index_from_int(&index.clone())
                            .unwrap_or_else(|error| __terrane_uncaught(
                                TerraneError::from(error)
                                    .at(
                                        "/standard/networking::lookup-dns (networking.trn:227:28)",
                                    ),
                            )),
                    })
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/networking::lookup-dns (networking.trn:227:28)",
                            ),
                    )),
            );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return DnsResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_int(&raw),
        false,
        candidates.clone(),
    );
}
