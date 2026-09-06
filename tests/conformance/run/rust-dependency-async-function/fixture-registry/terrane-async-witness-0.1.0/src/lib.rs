use std::pin::Pin;
use std::task::{Context, Poll};

struct YieldOnce(bool);

impl Future for YieldOnce {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[derive(Debug)]
pub struct ValidationError;

impl std::fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("fixture rejected input")
    }
}

impl std::error::Error for ValidationError {}

pub async fn echo_after_yield(value: String) -> String {
    YieldOnce(false).await;
    value
}

pub async fn checked_echo(value: String) -> Result<String, ValidationError> {
    YieldOnce(false).await;
    if value == "reject" {
        Err(ValidationError)
    } else {
        Ok(value)
    }
}

pub async fn ready_after_yield() -> String {
    YieldOnce(false).await;
    "ready".to_owned()
}

pub async fn timer_poll_count() -> u64 {
    let mut polls = 0_u64;
    let mut delay = Box::pin(tokio::time::sleep(std::time::Duration::from_millis(1000)));
    std::future::poll_fn(|context| {
        polls += 1;
        delay.as_mut().poll(context)
    })
    .await;
    polls
}

pub async fn socket_round_trip() -> String {
    use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .expect("fixture listener must bind");
    let address = listener
        .local_addr()
        .expect("fixture listener must have an address");
    let client = async move {
        let mut stream = tokio::net::TcpStream::connect(address)
            .await
            .expect("fixture client must connect");
        stream
            .write_all(b"awake")
            .await
            .expect("fixture client must write");
    };
    let server = async {
        let (mut stream, _) = listener.accept().await.expect("fixture must accept");
        let mut bytes = [0_u8; 5];
        stream
            .read_exact(&mut bytes)
            .await
            .expect("fixture server must read");
        String::from_utf8(bytes.to_vec()).expect("fixture payload must be UTF-8")
    };
    let ((), message) = tokio::join!(client, server);
    message
}
