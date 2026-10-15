use super::{ResultValue, TcpListener, TcpStream, UdpSocket, io_error};
use parking_lot::{Mutex, RwLock};
use std::io;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub(super) struct TcpSettings {
    pub no_delay: bool,
    pub ttl: Option<u32>,
}

impl TcpSettings {
    pub fn apply_std(self, stream: &TcpStream) -> Result<(), ResultValue> {
        stream
            .set_nodelay(self.no_delay)
            .map_err(|error| io_error("TCP no-delay configuration", &error))?;
        if let Some(ttl) = self.ttl {
            stream
                .set_ttl(ttl)
                .map_err(|error| io_error("TCP TTL configuration", &error))?;
        }
        Ok(())
    }

    pub fn apply_async(self, stream: &tokio::net::TcpStream) -> Result<(), ResultValue> {
        stream
            .set_nodelay(self.no_delay)
            .map_err(|error| io_error("TCP no-delay configuration", &error))?;
        if let Some(ttl) = self.ttl {
            stream
                .set_ttl(ttl)
                .map_err(|error| io_error("TCP TTL configuration", &error))?;
        }
        Ok(())
    }
}

// Detach a registered transport when uniquely owned, or register from a duplicate while
// retaining the original socket until registration succeeds.
fn take_async<S, T>(
    readiness: &Mutex<Option<Arc<T>>>,
    socket: &RwLock<Option<S>>,
    register: impl FnOnce(&S) -> Result<T, ResultValue>,
    busy_message: &'static str,
    closed_message: &'static str,
) -> Result<T, ResultValue> {
    let mut registered = readiness.lock();
    if let Some(value) = registered.take() {
        return match Arc::try_unwrap(value) {
            Ok(value) => {
                socket.write().take();
                Ok(value)
            }
            Err(value) => {
                *registered = Some(value);
                Err(ResultValue::error(busy_message))
            }
        };
    }

    let mut socket = socket.write();
    let original = socket
        .as_ref()
        .ok_or_else(|| ResultValue::error(closed_message))?;
    let value = register(original)?;
    socket.take();
    Ok(value)
}

fn register_listener(socket: &TcpListener) -> Result<tokio::net::TcpListener, ResultValue> {
    let duplicate = socket
        .try_clone()
        .map_err(|error| io_error("TCP listener readiness registration", &error))?;
    duplicate
        .set_nonblocking(true)
        .map_err(|error| io_error("TCP listener readiness configuration", &error))?;
    tokio::net::TcpListener::from_std(duplicate)
        .map_err(|error| io_error("TCP listener readiness registration", &error))
}

fn register_stream(socket: &TcpStream) -> Result<tokio::net::TcpStream, ResultValue> {
    let duplicate = socket
        .try_clone()
        .map_err(|error| io_error("TCP stream readiness registration", &error))?;
    duplicate
        .set_nonblocking(true)
        .map_err(|error| io_error("TCP stream readiness configuration", &error))?;
    tokio::net::TcpStream::from_std(duplicate)
        .map_err(|error| io_error("TCP stream readiness registration", &error))
}

pub(super) struct ListenerState {
    socket: RwLock<Option<TcpListener>>,
    readiness: Mutex<Option<Arc<tokio::net::TcpListener>>>,
    pub accepted: TcpSettings,
}

impl ListenerState {
    pub fn new(socket: TcpListener, accepted: TcpSettings) -> Self {
        Self {
            socket: RwLock::new(Some(socket)),
            readiness: Mutex::new(None),
            accepted,
        }
    }

    pub fn readiness(&self) -> Result<Arc<tokio::net::TcpListener>, ResultValue> {
        let mut registered = self.readiness.lock();
        if let Some(socket) = registered.as_ref() {
            return Ok(Arc::clone(socket));
        }
        let socket = self.socket.read();
        let socket = socket
            .as_ref()
            .ok_or_else(|| ResultValue::error("listener is closed"))?;
        let duplicate = socket
            .try_clone()
            .map_err(|error| io_error("TCP listener registration", &error))?;
        duplicate
            .set_nonblocking(true)
            .map_err(|error| io_error("TCP listener registration", &error))?;
        let socket = Arc::new(
            tokio::net::TcpListener::from_std(duplicate)
                .map_err(|error| io_error("TCP listener registration", &error))?,
        );
        *registered = Some(Arc::clone(&socket));
        Ok(socket)
    }

    pub fn take_async(&self) -> Result<tokio::net::TcpListener, ResultValue> {
        if tokio::runtime::Handle::try_current().is_err() {
            return Err(ResultValue::error("no Tokio runtime is active"));
        }
        take_async(
            &self.readiness,
            &self.socket,
            register_listener,
            "TCP listener is busy",
            "listener is closed",
        )
    }

    pub fn close(&self) {
        self.readiness.lock().take();
        self.socket.write().take();
    }
}

impl Deref for ListenerState {
    type Target = RwLock<Option<TcpListener>>;
    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

pub(super) struct TcpState {
    socket: RwLock<Option<TcpStream>>,
    readiness: Mutex<Option<Arc<tokio::net::TcpStream>>>,
}

impl TcpState {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket: RwLock::new(Some(socket)),
            readiness: Mutex::new(None),
        }
    }

    pub fn from_async(socket: tokio::net::TcpStream) -> Self {
        Self {
            socket: RwLock::new(None),
            readiness: Mutex::new(Some(Arc::new(socket))),
        }
    }

    pub fn is_open(&self) -> bool {
        self.readiness.lock().is_some() || self.socket.read().is_some()
    }

    pub fn configure(&self, settings: TcpSettings) -> Result<(), ResultValue> {
        let registered = self.readiness.lock();
        if let Some(socket) = registered.as_ref() {
            return settings.apply_async(socket);
        }
        let socket = self.socket.read();
        settings.apply_std(
            socket
                .as_ref()
                .ok_or_else(|| ResultValue::error("stream is closed"))?,
        )
    }

    pub fn shutdown(&self, direction: std::net::Shutdown) -> io::Result<()> {
        let registered = self.readiness.lock();
        if let Some(socket) = registered.as_ref() {
            return socket2::SockRef::from(socket.as_ref()).shutdown(direction);
        }
        let socket = self.socket.read();
        socket
            .as_ref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "stream is closed"))?
            .shutdown(direction)
    }

    pub fn take_std(&self) -> Result<TcpStream, ResultValue> {
        let mut registered = self.readiness.lock();
        if registered
            .as_ref()
            .is_some_and(|socket| Arc::strong_count(socket) != 1)
        {
            return Err(ResultValue::error("TCP stream is busy"));
        }
        let readiness = registered.take();
        let socket = if let Some(socket) = self.socket.write().take() {
            drop(readiness);
            socket
        } else if let Some(socket) = readiness {
            Arc::try_unwrap(socket)
                .map_err(|_| ResultValue::error("TCP stream is busy"))?
                .into_std()
                .map_err(|error| io_error("TCP transport conversion", &error))?
        } else {
            return Err(ResultValue::error("stream is closed"));
        };
        socket
            .set_nonblocking(false)
            .map_err(|error| io_error("TLS blocking configuration", &error))?;
        Ok(socket)
    }

    pub fn readiness(&self) -> Result<Arc<tokio::net::TcpStream>, ResultValue> {
        let mut registered = self.readiness.lock();
        if let Some(socket) = registered.as_ref() {
            return Ok(Arc::clone(socket));
        }
        let socket = self.socket.read();
        let socket = socket
            .as_ref()
            .ok_or_else(|| ResultValue::error("stream is closed"))?;
        let duplicate = socket
            .try_clone()
            .map_err(|error| io_error("TCP registration", &error))?;
        duplicate
            .set_nonblocking(true)
            .map_err(|error| io_error("TCP registration", &error))?;
        let socket = Arc::new(
            tokio::net::TcpStream::from_std(duplicate)
                .map_err(|error| io_error("TCP registration", &error))?,
        );
        *registered = Some(Arc::clone(&socket));
        Ok(socket)
    }

    pub fn take_async(&self) -> Result<tokio::net::TcpStream, ResultValue> {
        take_async(
            &self.readiness,
            &self.socket,
            register_stream,
            "TCP stream is busy",
            "stream is closed",
        )
    }

    pub fn take_native_async(&self) -> Result<tokio::net::TcpStream, ResultValue> {
        if tokio::runtime::Handle::try_current().is_err() {
            return Err(ResultValue::error("no Tokio runtime is active"));
        }
        self.take_async()
    }

    pub fn close(&self) {
        self.readiness.lock().take();
        self.socket.write().take();
    }
}

impl Deref for TcpState {
    type Target = RwLock<Option<TcpStream>>;
    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

// Cloning a registered socket does not provide a separate blocking descriptor: O_NONBLOCK is
// shared by the underlying file description. Synchronous UDP therefore waits on readiness and
// never changes socket-wide mode or timeout options after async registration.
pub(super) struct UdpState {
    socket: Mutex<Option<UdpSocket>>,
    readiness: Mutex<Option<Arc<tokio::net::UdpSocket>>>,
}

impl UdpState {
    pub fn new(socket: UdpSocket) -> Self {
        Self {
            socket: Mutex::new(Some(socket)),
            readiness: Mutex::new(None),
        }
    }
    pub fn readiness(&self) -> Result<Arc<tokio::net::UdpSocket>, ResultValue> {
        let mut registered = self.readiness.lock();
        if let Some(socket) = registered.as_ref() {
            return Ok(Arc::clone(socket));
        }
        let socket = self.socket.lock();
        let socket = socket
            .as_ref()
            .ok_or_else(|| ResultValue::error("socket is closed"))?;
        let duplicate = socket
            .try_clone()
            .map_err(|error| io_error("UDP registration", &error))?;
        duplicate
            .set_nonblocking(true)
            .map_err(|error| io_error("UDP registration", &error))?;
        let socket = Arc::new(
            tokio::net::UdpSocket::from_std(duplicate)
                .map_err(|error| io_error("UDP registration", &error))?,
        );
        *registered = Some(Arc::clone(&socket));
        Ok(socket)
    }
    pub fn close(&self) {
        self.readiness.lock().take();
        self.socket.lock().take();
    }
}

impl Deref for UdpState {
    type Target = Mutex<Option<UdpSocket>>;
    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

pub(super) async fn read(stream: &tokio::net::TcpStream, bytes: &mut [u8]) -> io::Result<usize> {
    loop {
        match stream.try_read(bytes) {
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => stream.readable().await?,
            result => return result,
        }
    }
}

pub(super) async fn write(stream: &tokio::net::TcpStream, bytes: &[u8]) -> io::Result<usize> {
    loop {
        match stream.try_write(bytes) {
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => stream.writable().await?,
            result => return result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TcpState, take_async};
    use std::io::{Read as _, Write as _};
    use std::net::{TcpListener, TcpStream};
    use std::time::Duration;

    #[test]
    fn failed_registration_preserves_original_socket() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut peer = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        let (stream, _) = listener.accept().unwrap();
        let state = TcpState::new(stream);
        let result = take_async::<_, tokio::net::TcpStream>(
            &state.readiness,
            &state.socket,
            |_| Err(crate::ResultValue::error("registration failed")),
            "busy",
            "closed",
        );
        assert!(result.is_err());
        let mut recovered = state
            .take_std()
            .unwrap_or_else(|error| panic!("{}", error.message));
        recovered
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        peer.write_all(b"retained").unwrap();
        let mut bytes = [0; 8];
        recovered.read_exact(&mut bytes).unwrap();
        assert_eq!(&bytes, b"retained");
    }
}
