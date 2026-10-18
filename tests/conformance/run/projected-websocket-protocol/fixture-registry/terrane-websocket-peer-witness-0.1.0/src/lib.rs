use std::future::Future;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::time::Duration;

#[derive(Debug)]
pub struct Error(String);
impl std::fmt::Display for Error {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        output.write_str(&self.0)
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self(error.to_string())
    }
}

pub struct CloseFrame {
    code: u16,
}
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Close(Option<CloseFrame>),
}
impl Message {
    pub fn text(value: String) -> Self {
        Self::Text(value)
    }
    pub fn into_text(self) -> Result<String, Error> {
        match self {
            Self::Text(text) => Ok(text),
            _ => Err(Error("not text".into())),
        }
    }
    pub fn into_data(self) -> Vec<u8> {
        match self {
            Self::Binary(data) | Self::Ping(data) | Self::Pong(data) => data,
            _ => panic!("not a data frame"),
        }
    }
}

pub struct WebSocket(TcpStream);
impl WebSocket {
    pub async fn recv(&mut self) -> Result<Option<Message>, Error> {
        read_frame(&mut self.0, true)
    }
    pub async fn send(&mut self, message: Message) -> Result<(), Error> {
        write_frame(&mut self.0, &message, false)
    }
}

// This bounded fixture implements the short, final RFC 6455 frames exercised below.
// HTTP upgrade/handshake and arbitrary WebSocket compatibility are not its contract.
fn read_frame(stream: &mut TcpStream, masked: bool) -> Result<Option<Message>, Error> {
    let mut header = [0; 2];
    if stream.read(&mut header[..1])? == 0 {
        return Ok(None);
    }
    stream.read_exact(&mut header[1..])?;
    if header[0] & 0xf0 != 0x80 || (header[1] & 0x80 != 0) != masked {
        return Err(Error("invalid frame flags".into()));
    }
    let length = usize::from(header[1] & 0x7f);
    if length > 125 {
        return Err(Error("fixture only admits short frames".into()));
    }
    let mut mask = [0; 4];
    if masked {
        stream.read_exact(&mut mask)?;
    }
    let mut data = vec![0; length];
    stream.read_exact(&mut data)?;
    if masked {
        for (index, byte) in data.iter_mut().enumerate() {
            *byte ^= mask[index % 4];
        }
    }
    Ok(Some(match header[0] & 0x0f {
        1 => Message::Text(String::from_utf8(data).map_err(|error| Error(error.to_string()))?),
        2 => Message::Binary(data),
        8 if data.is_empty() => Message::Close(None),
        9 => Message::Ping(data),
        10 => Message::Pong(data),
        _ => return Err(Error("invalid frame opcode or close payload".into())),
    }))
}

fn write_frame(stream: &mut TcpStream, message: &Message, masked: bool) -> Result<(), Error> {
    let close;
    let (opcode, data): (u8, &[u8]) = match message {
        Message::Text(text) => (1, text.as_bytes()),
        Message::Binary(data) => (2, data),
        Message::Ping(data) => (9, data),
        Message::Pong(data) => (10, data),
        Message::Close(None) => (8, &[]),
        Message::Close(Some(frame)) => {
            close = frame.code.to_be_bytes();
            (8, &close)
        }
    };
    assert!(data.len() <= 125);
    stream.write_all(&[
        0x80 | opcode,
        data.len() as u8 | if masked { 0x80 } else { 0 },
    ])?;
    if masked {
        let mask = [1, 2, 3, 4];
        stream.write_all(&mask)?;
        let mut encoded = [0; 125];
        for (index, byte) in data.iter().enumerate() {
            encoded[index] = byte ^ mask[index % 4];
        }
        stream.write_all(&encoded[..data.len()])?;
    } else {
        stream.write_all(data)?;
    }
    Ok(())
}

fn connect(address: std::net::SocketAddr) -> TcpStream {
    let stream = TcpStream::connect(address).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    stream
}

fn drive_peer(address: std::net::SocketAddr) {
    let mut stream = connect(address);
    write_frame(&mut stream, &Message::Text("hello".into()), true).unwrap();
    assert!(
        matches!(read_frame(&mut stream, false).unwrap(), Some(Message::Text(text)) if text == "hello")
    );
    write_frame(&mut stream, &Message::Binary(vec![1, 2]), true).unwrap();
    assert!(
        matches!(read_frame(&mut stream, false).unwrap(), Some(Message::Binary(data)) if data == [1, 2])
    );
    write_frame(&mut stream, &Message::Ping(vec![3]), true).unwrap();
    assert!(
        matches!(read_frame(&mut stream, false).unwrap(), Some(Message::Pong(data)) if data == [3])
    );
    write_frame(&mut stream, &Message::Pong(vec![4]), true).unwrap();
    write_frame(&mut stream, &Message::Text("shutdown".into()), true).unwrap();
    assert!(matches!(
        read_frame(&mut stream, false).unwrap(),
        Some(Message::Close(None))
    ));
    drop(stream);

    let mut stream = connect(address);
    write_frame(&mut stream, &Message::Close(None), true).unwrap();
    stream.shutdown(Shutdown::Write).unwrap();
    let mut remaining = Vec::new();
    stream.read_to_end(&mut remaining).unwrap();
    assert!(remaining.is_empty());

    let mut stream = connect(address);
    // A masked frame with an invalid opcode must reach the Terrane typed catch.
    stream.write_all(&[0x8f, 0x80, 1, 2, 3, 4]).unwrap();
}

pub async fn exercise<F, Fut>(handler: F)
where
    F: Fn(WebSocket) -> Fut + Send,
    Fut: Future<Output = ()> + Send,
{
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let peer = std::thread::spawn(move || drive_peer(address));
    for _ in 0..3 {
        let (stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        handler(WebSocket(stream)).await;
    }
    peer.join().unwrap();
}
