use super::{Arc, ResultValue, TcpListener, TcpStream, UdpSocket, io_error};
use parking_lot::{Mutex, RwLock};
use std::io;
use std::ops::Deref;

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

// Synchronous construction does not require an executor. The first asynchronous operation
// installs one registration, retained until close or a consuming transport upgrade.
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
        let mut registered = self.readiness.lock();
        if let Some(socket) = registered.take() {
            return match Arc::try_unwrap(socket) {
                Ok(socket) => {
                    self.socket.write().take();
                    Ok(socket)
                }
                Err(socket) => {
                    *registered = Some(socket);
                    Err(ResultValue::error("TCP stream is busy"))
                }
            };
        }
        let socket = self
            .socket
            .write()
            .take()
            .ok_or_else(|| ResultValue::error("stream is closed"))?;
        socket
            .set_nonblocking(true)
            .map_err(|error| io_error("TLS readiness configuration", &error))?;
        tokio::net::TcpStream::from_std(socket)
            .map_err(|error| io_error("TLS readiness registration", &error))
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
