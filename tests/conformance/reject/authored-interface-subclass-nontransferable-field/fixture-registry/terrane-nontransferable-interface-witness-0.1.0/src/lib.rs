use std::rc::Rc;

#[derive(Clone)]
pub struct NonSend(Rc<()>);

pub fn make_non_send() -> NonSend {
    NonSend(Rc::new(()))
}
