pub struct Counter {
    value: i64,
}

impl Counter {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn finish(self: Box<Self>) -> impl Future<Output = i64> + Send {
        async move { self.value }
    }
}

#[derive(Clone)]
pub struct OpaqueValue {
    value: i64,
}

impl OpaqueValue {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

pub async fn checksum(values: &[OpaqueValue]) -> i64 {
    let mut yielded = false;
    std::future::poll_fn(|cx| {
        if yielded {
            return std::task::Poll::Ready(());
        }
        yielded = true;
        cx.waker().wake_by_ref();
        std::task::Poll::Pending
    })
    .await;
    values.iter().map(|value| value.value).sum()
}

pub struct OpaqueRow {
    value: i64,
}

impl OpaqueRow {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

pub fn rows() -> Vec<OpaqueRow> {
    vec![OpaqueRow::new(11), OpaqueRow::new(17)]
}

pub fn sum(values: &[i32]) -> i64 {
    values.iter().map(|&value| i64::from(value)).sum()
}
