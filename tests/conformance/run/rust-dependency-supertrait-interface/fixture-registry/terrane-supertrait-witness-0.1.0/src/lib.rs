pub trait Base: Send + Sync {
    fn base(&self) -> i64;
}

pub trait Child: Base {
    fn child(&self) -> i64;
}

pub fn child_total<T: Child>(value: T) -> i64 {
    value.base() + value.child()
}

pub fn erased_child_total(value: Box<dyn Child + Send + Sync>) -> i64 {
    value.base() + value.child()
}
