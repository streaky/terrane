// Vendored support crates: terrane-scalar-support
// Source: core/networking_native.trn
// Namespace: core/networking/native
pub type NativeTcpListener = tokio::net::TcpListener;
pub type NativeTcpStream = tokio::net::TcpStream;
pub type NativeSocketAddress = std::net::SocketAddr;
pub type HostListenerTransfer = std::io::Result<tokio::net::TcpListener>;
pub type HostStreamTransfer = std::io::Result<tokio::net::TcpStream>;
