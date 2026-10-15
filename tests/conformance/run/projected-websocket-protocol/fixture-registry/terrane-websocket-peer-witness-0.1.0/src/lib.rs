use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::tungstenite::{Bytes, Message};

async fn verify_health(address: &str) {
    let mut stream = tokio::net::TcpStream::connect(address).await.unwrap();
    stream
        .write_all(
            format!("GET /health HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n")
                .as_bytes(),
        )
        .await
        .unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    let response = String::from_utf8(response).unwrap();
    assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(response.ends_with("\r\n\r\nok"));
}

async fn connect(
    address: &str,
) -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
    tokio_tungstenite::connect_async(format!("ws://{address}/ws"))
        .await
        .expect("WebSocket fixture server did not accept connection")
        .0
}

pub async fn drive_peer(address: String) {
    verify_health(&address).await;

    let mut socket = connect(&address).await;

    socket.send(Message::Text("hello".into())).await.unwrap();
    assert!(matches!(socket.next().await, Some(Ok(Message::Text(value))) if value == "hello"));

    socket
        .send(Message::Binary(Bytes::from_static(&[1, 2])))
        .await
        .unwrap();
    assert!(
        matches!(socket.next().await, Some(Ok(Message::Binary(value))) if value.as_ref() == [1, 2])
    );

    socket
        .send(Message::Ping(Bytes::from_static(&[3])))
        .await
        .unwrap();
    while !matches!(socket.next().await, Some(Ok(Message::Pong(_)))) {}

    socket
        .send(Message::Pong(Bytes::from_static(&[4])))
        .await
        .unwrap();
    socket.send(Message::Text("shutdown".into())).await.unwrap();
    while let Some(message) = socket.next().await {
        if matches!(message, Ok(Message::Close(_))) {
            break;
        }
    }

    let mut socket = connect(&address).await;
    socket.close(None).await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;

    let socket = connect(&address).await;
    drop(socket);
    tokio::time::sleep(Duration::from_millis(100)).await;
}
