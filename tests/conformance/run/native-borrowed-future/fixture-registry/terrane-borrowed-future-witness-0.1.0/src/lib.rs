use std::io;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Builder {
    value: u8,
}

impl Builder {
    pub fn new() -> Self {
        Self { value: 1 }
    }

    pub fn set(&mut self, value: u8) {
        self.value = value;
    }

    pub fn serve(&self) -> Connection<'_, u8> {
        Connection {
            lender: self,
            yielded: false,
            marker: PhantomData,
            padding: [0; 64 * 1024],
        }
    }
}

pub struct Connection<'a, T> {
    lender: &'a Builder,
    yielded: bool,
    marker: PhantomData<fn() -> T>,
    padding: [u8; 64 * 1024],
}

impl<T> Future for Connection<'_, T> {
    type Output = Result<u8, io::Error>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let value = self.lender.value + self.padding[0];
        if !self.yielded {
            self.yielded = true;
            context.waker().wake_by_ref();
            return Poll::Pending;
        }
        Poll::Ready(Ok(value))
    }
}

pub struct Envelope<T> {
    value: T,
}

impl<T> Envelope<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn into_value(self) -> T {
        self.value
    }
}
