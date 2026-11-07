use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug)]
pub struct SinkError(&'static str);
impl Display for SinkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for SinkError {}

pub struct Incoming {
    receiver: tokio::sync::mpsc::Receiver<String>,
}
impl Incoming {
    pub async fn next(&mut self) -> Result<Option<String>, SinkError> {
        Ok(self.receiver.recv().await)
    }
    pub async fn close(self) -> Result<(), SinkError> {
        Ok(())
    }
}

pub struct Outgoing {
    sender: tokio::sync::mpsc::Sender<String>,
}
impl Outgoing {
    pub async fn send(&mut self, value: String) -> Result<bool, SinkError> {
        Ok(self.sender.send(value).await.is_ok())
    }
    pub async fn flush(&mut self) -> Result<bool, SinkError> {
        Ok(true)
    }
    pub async fn close(self) -> Result<bool, SinkError> {
        Ok(true)
    }
}

pub struct SplitEndpoints {
    incoming: Option<Incoming>,
    outgoing: Option<Outgoing>,
}
impl SplitEndpoints {
    pub fn take_incoming(&mut self) -> Result<Incoming, SinkError> {
        self.incoming
            .take()
            .ok_or(SinkError("incoming endpoint already taken"))
    }
    pub fn take_outgoing(self) -> Result<Outgoing, SinkError> {
        self.outgoing
            .ok_or(SinkError("outgoing endpoint already taken"))
    }
}

pub struct Duplex {
    capacity: usize,
}
impl Duplex {
    pub fn split(self) -> Result<SplitEndpoints, SinkError> {
        let (sender, receiver) = tokio::sync::mpsc::channel(self.capacity);
        Ok(SplitEndpoints {
            incoming: Some(Incoming { receiver }),
            outgoing: Some(Outgoing { sender }),
        })
    }
}
pub fn duplex(capacity: i64) -> Duplex {
    Duplex {
        capacity: usize::try_from(capacity).expect("positive fixture capacity"),
    }
}

pub async fn drain_slowly(mut incoming: Incoming) -> String {
    let mut values = Vec::new();
    while let Some(value) = incoming.receiver.recv().await {
        values.push(value);
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    values.join("|")
}

pub fn remotely_closed_sink() -> Outgoing {
    let (sender, receiver) = tokio::sync::mpsc::channel(1);
    drop(receiver);
    Outgoing { sender }
}

pub fn blocked_sink() -> Outgoing {
    let (sender, mut receiver) = tokio::sync::mpsc::channel(1);
    sender
        .try_send("occupied".to_owned())
        .expect("fixture capacity");
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(60)).await;
        receiver.recv().await;
    });
    Outgoing { sender }
}

pub struct QueueSink {
    values: VecDeque<String>,
}
impl QueueSink {
    pub async fn send(&mut self, value: String) -> Result<bool, SinkError> {
        self.values.push_back(value);
        Ok(true)
    }
    pub fn flush(&mut self) -> Result<i64, SinkError> {
        Ok(self.values.len() as i64)
    }
    pub fn close(self) -> Result<String, SinkError> {
        Ok(self.values.into_iter().collect::<Vec<_>>().join("|"))
    }
}
pub fn queue_sink() -> QueueSink {
    QueueSink {
        values: VecDeque::new(),
    }
}
