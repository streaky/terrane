use std::io::ErrorKind;
use terrane_platform_support::{NativeTcpListener, NativeTcpStream};
async fn read_exact(stream: &NativeTcpStream, mut buffer: &mut [u8]) {
    while !buffer.is_empty() {
        stream.readable().await.unwrap();
        match stream.try_read(buffer) {
            Ok(0) => panic!("unexpected EOF"),
            Ok(count) => buffer = &mut buffer[count..],
            Err(error) if error.kind() == ErrorKind::WouldBlock => {}
            Err(error) => panic!("{error}"),
        }
    }
}
async fn write_all(stream: &NativeTcpStream, mut buffer: &[u8]) {
    while !buffer.is_empty() {
        stream.writable().await.unwrap();
        match stream.try_write(buffer) {
            Ok(0) => panic!("zero-length write"),
            Ok(count) => buffer = &buffer[count..],
            Err(error) if error.kind() == ErrorKind::WouldBlock => {}
            Err(error) => panic!("{error}"),
        }
    }
}
async fn expect_eof(stream: &NativeTcpStream) {
    loop {
        stream.readable().await.unwrap();
        match stream.try_read(&mut [0; 1]) {
            Ok(0) => return,
            Ok(_) => panic!("unexpected trailing data"),
            Err(error) if error.kind() == ErrorKind::WouldBlock => {}
            Err(error) => panic!("{error}"),
        }
    }
}
pub async fn serve_one(listener: &NativeTcpListener) {
    let (stream, _) = listener.accept().await.unwrap();
    assert!(stream.nodelay().unwrap());
    assert_eq!(stream.ttl().unwrap(), 23);
    let mut request = [0; 4];
    read_exact(&stream, &mut request).await;
    assert_eq!(&request, b"ping");
    write_all(&stream, b"pong").await;
    socket2::SockRef::from(&stream).shutdown(std::net::Shutdown::Write).unwrap();
    read_exact(&stream, &mut request).await;
    assert_eq!(&request, b"tail");
    expect_eof(&stream).await;
}
pub async fn exchange(stream: &NativeTcpStream) -> String {
    write_all(stream, b"ping").await;
    let mut response = [0; 4];
    read_exact(stream, &mut response).await;
    expect_eof(stream).await;
    write_all(stream, b"tail").await;
    socket2::SockRef::from(stream).shutdown(std::net::Shutdown::Write).unwrap();
    String::from_utf8(response.to_vec()).unwrap()
}
