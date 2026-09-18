use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::{Bytes, Message};

async fn connect() -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
    for _ in 0..100 {
        if let Ok((socket, _)) = tokio_tungstenite::connect_async("ws://127.0.0.1:38765/ws").await {
            return socket;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("WebSocket fixture server did not start");
}

pub async fn drive_peer() {
    let mut socket = connect().await;

    socket.send(Message::Text("hello".into())).await.unwrap();
    assert!(matches!(socket.next().await, Some(Ok(Message::Text(value))) if value == "hello"));

    socket
        .send(Message::Binary(Bytes::from_static(&[1, 2])))
        .await
        .unwrap();
    assert!(matches!(socket.next().await, Some(Ok(Message::Binary(value))) if value.as_ref() == [1, 2]));

    socket
        .send(Message::Ping(Bytes::from_static(&[3])))
        .await
        .unwrap();
    while !matches!(socket.next().await, Some(Ok(Message::Pong(_)))) {}

    socket
        .send(Message::Pong(Bytes::from_static(&[4])))
        .await
        .unwrap();
    socket.send(Message::Close(None)).await.unwrap();
    while let Some(message) = socket.next().await {
        if matches!(message, Ok(Message::Close(_))) {
            break;
        }
    }

    let socket = connect().await;
    drop(socket);
    tokio::time::sleep(Duration::from_millis(100)).await;
}
