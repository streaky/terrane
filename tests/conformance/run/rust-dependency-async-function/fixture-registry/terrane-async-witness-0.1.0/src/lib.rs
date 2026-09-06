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
