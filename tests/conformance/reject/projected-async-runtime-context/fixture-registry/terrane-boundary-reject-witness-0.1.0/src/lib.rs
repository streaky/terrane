use std::rc::Rc;

#[derive(Clone)]
pub struct NonSend(pub Rc<()>);

pub fn non_send() -> NonSend {
    NonSend(Rc::new(()))
}

pub struct UniqueToken;
pub fn unique_token() -> UniqueToken {
    UniqueToken
}

pub trait LocalWorker {
    fn run(&self) -> i64;
}
pub fn retain_local<T: LocalWorker + Send + 'static>(_value: T) {}

#[allow(drop_bounds)]
pub trait DropWorker: Drop {
    fn run(&self) -> i64;
}

#[allow(async_fn_in_trait)]
pub trait AsyncWorker: Send {
    async fn run(&self) -> i64;
}

pub fn borrowed_worker<'a>(value: &'a dyn LocalWorker) -> i64 {
    value.run()
}
pub fn boxed_string(value: Box<String>) -> usize {
    value.len()
}
pub fn boxed_worker_result() -> Box<dyn LocalWorker> {
    panic!("projection-only")
}

#[allow(async_fn_in_trait)]
pub trait LocalAsync {
    async fn run(&self) -> i64;
}

pub use std::ops::Drop as CanonicalDrop;
