pub trait Base: Send + Sync {
    fn base(&self) -> i64 {
        10
    }
}

pub trait Middle: Base {
    fn middle(&self) -> i64;
}

pub trait Child: Middle {
    fn child(&self) -> i64;
}

pub trait Left: Base {
    fn left(&self) -> i64;
}

pub trait Right: Base {
    fn right(&self) -> i64;
}

pub trait Diamond: Left + Right {
    fn diamond(&self) -> i64;
}

pub fn child_total<T: Child>(value: T) -> i64 {
    value.base() + value.middle() + value.child()
}

pub fn erased_child_total(value: Box<dyn Child + Send + Sync>) -> i64 {
    value.base() + value.middle() + value.child()
}

pub fn diamond_total<T: Diamond>(value: T) -> i64 {
    value.base() + value.left() + value.right() + value.diamond()
}
