#[derive(Clone)]
pub struct Foreign;

pub fn foreign() -> Foreign {
    Foreign
}

pub trait Worker: Send {
    fn value(&self) -> i64;
}
