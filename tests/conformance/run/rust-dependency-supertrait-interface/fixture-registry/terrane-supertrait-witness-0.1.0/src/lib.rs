pub trait Base: Send + Sync {
    fn base(&self) -> i64;
}

pub trait Middle: Base {
    fn middle(&self) -> i64;
}

pub trait Child: Middle {
    fn child(&self) -> i64;
}

pub fn child_total<T: Child>(value: T) -> i64 {
    value.base() + value.middle() + value.child()
}

pub fn erased_child_total(value: Box<dyn Child + Send + Sync>) -> i64 {
    value.base() + value.middle() + value.child()
}
