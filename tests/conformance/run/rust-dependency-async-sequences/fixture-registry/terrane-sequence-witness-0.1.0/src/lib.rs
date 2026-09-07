use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use tokio_stream::{StreamExt, wrappers::ReceiverStream};

#[derive(Debug)]
pub struct SequenceError(&'static str);
impl Display for SequenceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}
impl std::error::Error for SequenceError {}

pub struct TokioSequence {
    stream: ReceiverStream<Result<i64, SequenceError>>,
}
impl TokioSequence {
    pub async fn next(&mut self) -> Result<Option<i64>, SequenceError> {
        match self.stream.next().await {
            Some(Ok(value)) => Ok(Some(value)),
            Some(Err(error)) => Err(error),
            None => Ok(None),
        }
    }
    pub async fn close(self) -> Result<bool, SequenceError> {
        Ok(true)
    }
}

pub fn make_tokio_sequence() -> TokioSequence {
    let (sender, receiver) = tokio::sync::mpsc::channel(4);
    sender.try_send(Ok(3)).expect("fixture capacity");
    sender.try_send(Ok(5)).expect("fixture capacity");
    drop(sender);
    TokioSequence {
        stream: ReceiverStream::new(receiver),
    }
}

pub fn make_failing_tokio_sequence() -> TokioSequence {
    let (sender, receiver) = tokio::sync::mpsc::channel(2);
    sender
        .try_send(Err(SequenceError("remote protocol failure")))
        .expect("fixture capacity");
    drop(sender);
    TokioSequence {
        stream: ReceiverStream::new(receiver),
    }
}

pub fn make_pending_tokio_sequence() -> TokioSequence {
    let (sender, receiver) = tokio::sync::mpsc::channel(1);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(60)).await;
        drop(sender);
    });
    TokioSequence {
        stream: ReceiverStream::new(receiver),
    }
}

pub struct QueueSequence {
    values: VecDeque<String>,
    failure: Option<SequenceError>,
    pending: bool,
}
impl QueueSequence {
    pub async fn next(&mut self) -> Result<Option<String>, SequenceError> {
        if self.pending {
            return std::future::pending().await;
        }
        if let Some(error) = self.failure.take() {
            return Err(error);
        }
        Ok(self.values.pop_front())
    }
    pub fn close(self) -> Result<bool, SequenceError> {
        Ok(true)
    }
}
pub fn make_queue_sequence() -> QueueSequence {
    QueueSequence {
        values: VecDeque::from(["different-one".to_owned(), "different-two".to_owned()]),
        failure: None,
        pending: false,
    }
}
pub fn make_failing_queue_sequence() -> QueueSequence {
    QueueSequence {
        values: VecDeque::new(),
        failure: Some(SequenceError("queue protocol failure")),
        pending: false,
    }
}
pub fn make_pending_queue_sequence() -> QueueSequence {
    QueueSequence {
        values: VecDeque::new(),
        failure: None,
        pending: true,
    }
}

pub struct BorrowedSequence {
    value: String,
}

impl BorrowedSequence {
    pub async fn next(&mut self) -> Result<Option<&str>, SequenceError> {
        Ok(Some(&self.value))
    }

    pub fn close(self) -> Result<(), SequenceError> {
        Ok(())
    }
}

pub fn make_borrowed_sequence() -> BorrowedSequence {
    BorrowedSequence {
        value: "borrowed".to_owned(),
    }
}

pub trait OpenSequence {
    type Item;

    fn next(
        &mut self,
    ) -> impl std::future::Future<Output = Result<Option<Self::Item>, SequenceError>>;
}

pub struct TcpSequence {
    listener: tokio::net::TcpListener,
    remaining: usize,
}

impl TcpSequence {
    pub async fn next(&mut self) -> Result<Option<String>, SequenceError> {
        if self.remaining == 0 {
            return Ok(None);
        }
        let (mut stream, _) = self
            .listener
            .accept()
            .await
            .map_err(|_| SequenceError("accept failure"))?;
        let mut bytes = Vec::new();
        tokio::io::AsyncReadExt::read_to_end(&mut stream, &mut bytes)
            .await
            .map_err(|_| SequenceError("read failure"))?;
        self.remaining -= 1;
        String::from_utf8(bytes)
            .map(Some)
            .map_err(|_| SequenceError("invalid UTF-8"))
    }

    pub async fn close(self) -> Result<bool, SequenceError> {
        Ok(true)
    }
}

pub async fn make_tcp_sequence() -> Result<TcpSequence, SequenceError> {
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .map_err(|_| SequenceError("bind failure"))?;
    let address = listener
        .local_addr()
        .map_err(|_| SequenceError("address failure"))?;
    tokio::spawn(async move {
        for value in ["network-one", "network-two"] {
            let mut stream = tokio::net::TcpStream::connect(address)
                .await
                .expect("loopback connect");
            tokio::io::AsyncWriteExt::write_all(&mut stream, value.as_bytes())
                .await
                .expect("loopback write");
        }
    });
    Ok(TcpSequence {
        listener,
        remaining: 2,
    })
}
