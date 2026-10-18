use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct Response {
    body: Vec<u8>,
}
impl Response {
    pub fn text(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body)
    }
}

pub fn get(address: String) -> Result<Response, std::io::Error> {
    let mut stream = TcpStream::connect(&address)?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    write!(
        stream,
        "GET / HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n"
    )?;
    let mut data = Vec::new();
    stream.read_to_end(&mut data)?;
    let split = data
        .windows(4)
        .position(|part| part == b"\r\n\r\n")
        .ok_or_else(|| std::io::Error::other("missing HTTP response headers"))?;
    data.drain(..split + 4);
    Ok(Response { body: data })
}
