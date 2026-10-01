#[derive(Clone)]
pub struct Foreign(std::rc::Rc<()>);

pub fn foreign() -> Foreign {
    Foreign(std::rc::Rc::new(()))
}

pub trait Worker: Send {
    fn value(&self) -> i64;
}
