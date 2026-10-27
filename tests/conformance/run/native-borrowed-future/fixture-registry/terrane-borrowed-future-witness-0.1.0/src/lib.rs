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

    pub async fn advance(&mut self) -> u8 {
        self.value += 1;
        self.value
    }

    pub async fn reject_value(&self) -> Result<u8, Rejection> {
        Err(Rejection)
    }

    pub fn serve(&self) -> Connection<'_, u8> {
        Connection {
            lender: self,
            yielded: false,
            marker: PhantomData,
            padding: [0; 64 * 1024],
        }
    }

    pub async fn reject(&self) -> Result<(), Rejection> {
        Err(Rejection)
    }
}
pub fn read_literal_later(value: &str) -> BorrowedLater<'_> {
    BorrowedLater { value, yielded: false }
}

pub type Text = str;

pub fn read_alias_later(value: &Text) -> BorrowedLater<'_> {
    read_literal_later(value)
}

pub struct BorrowedLater<'a> {
    value: &'a str,
    yielded: bool,
}

impl<'a> Future for BorrowedLater<'a> {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded {
            Poll::Ready(self.value.to_owned())
        } else {
            self.yielded = true;
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

pub async fn read_owned_later(value: String) -> String {
    YieldOnce(false).await;
    value
}

#[derive(Clone)]
pub struct TextQuery<'text> {
    value: &'text str,
}

pub fn literal_query(value: &Text) -> TextQuery<'_> {
    TextQuery { value }
}

impl TextQuery<'_> {
    pub async fn fetch(self) -> String {
        YieldOnce(false).await;
        self.value.to_owned()
    }
}

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


#[derive(Clone)]
pub struct Options;

impl Options {
    pub fn new() -> Self {
        Self
    }

    pub fn with_label(self, label: impl AsRef<str>) -> String {
        label.as_ref().to_owned()
    }

    pub fn with_owned_label(self, label: String) -> String {
        label
    }

    pub fn with_borrowed_label(self, label: &str) -> String {
        label.to_owned()
    }

    pub fn with_string_ref(self, label: &String) -> String {
        label.clone()
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

#[derive(Debug)]
pub struct Rejection;

impl std::fmt::Display for Rejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("native rejection")
    }
}

impl std::error::Error for Rejection {}

pub fn make_rejection() -> Rejection {
    Rejection
}

#[derive(Debug)]
pub struct UnimportedRejection;

impl std::fmt::Display for UnimportedRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("unimported native rejection")
    }
}

impl std::error::Error for UnimportedRejection {}

pub async fn reject_free() -> Result<(), UnimportedRejection> {
    Err(UnimportedRejection)
}
