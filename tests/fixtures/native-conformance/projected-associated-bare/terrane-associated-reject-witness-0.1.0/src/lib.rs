use std::rc::Rc;

#[derive(Clone)]
pub struct NonSend(pub Rc<()>);

pub fn non_send() -> NonSend {
    NonSend(Rc::new(()))
}

pub trait Sequence: Send + Sync {
    type Item: Clone + Send + Sync + 'static;
    fn first(&self) -> Option<Self::Item>;
    fn length(&self) -> i64;
}

pub fn sequence_total<T: Sequence<Item = i64>>(value: T) -> i64 {
    value.length()
}

pub trait SendItem {
    type Item: Send;
    fn item(&self) -> Self::Item;
}

pub trait TwoSlots {
    type Left;
    type Right;
    fn left(&self) -> Self::Left;
}

pub trait GenericSlot {
    type Item<'a>
    where
        Self: 'a;
    fn item(&self) -> Self::Item<'_>;
}

pub fn erased_borrowed(value: Box<dyn Sequence<Item = &'static str>>) -> i64 {
    value.length()
}
