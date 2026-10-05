pub struct Opaque<T>(pub T);
pub struct BadValue(std::rc::Rc<()>);

pub fn make_value() -> Opaque<BadValue> {
    Opaque(BadValue(std::rc::Rc::new(())))
}

pub trait Worker: Send {
    fn work(&self) -> bool;
}
