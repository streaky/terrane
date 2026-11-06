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
struct PendingOperation;

impl Future for PendingOperation {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl Drop for PendingOperation {
    fn drop(&mut self) {
        OPERATION_DROPS.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

static OPERATION_STARTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static OPERATION_DROPS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

pub fn reset_operation_state() {
    OPERATION_STARTED.store(false, std::sync::atomic::Ordering::Release);
    OPERATION_DROPS.store(0, std::sync::atomic::Ordering::Release);
}

pub async fn wait_forever() -> String {
    OPERATION_STARTED.store(true, std::sync::atomic::Ordering::Release);
    PendingOperation.await;
    unreachable!("pending operation completed")
}

pub async fn wait_until_operation_started() -> bool {
    while !OPERATION_STARTED.load(std::sync::atomic::Ordering::Acquire) {
        YieldOnce(false).await;
    }
    true
}

pub fn operation_drop_count() -> u64 {
    OPERATION_DROPS.load(std::sync::atomic::Ordering::Acquire)
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

static SIBLING_PROGRESS: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(0);

pub async fn wait_for_sibling() -> String {
    SIBLING_PROGRESS.store(1, std::sync::atomic::Ordering::Release);
    while SIBLING_PROGRESS.load(std::sync::atomic::Ordering::Acquire) != 2 {
        YieldOnce(false).await;
    }
    "interleaved".to_owned()
}

pub async fn signal_sibling() -> String {
    while SIBLING_PROGRESS.load(std::sync::atomic::Ordering::Acquire) != 1 {
        YieldOnce(false).await;
    }
    SIBLING_PROGRESS.store(2, std::sync::atomic::Ordering::Release);
    "signalled".to_owned()
}
