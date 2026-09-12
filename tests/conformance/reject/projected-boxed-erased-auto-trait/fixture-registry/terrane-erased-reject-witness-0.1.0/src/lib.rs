pub trait LocalWorker {
    fn run(&self) -> i64;
}

pub fn retain_boxed_send(_value: Box<dyn LocalWorker + Send>) {}
