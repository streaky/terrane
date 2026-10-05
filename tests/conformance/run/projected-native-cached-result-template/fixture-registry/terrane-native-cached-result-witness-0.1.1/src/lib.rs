use std::marker::PhantomData;

pub struct Lane<S = (), E = std::convert::Infallible> {
    value: i64,
    _state: PhantomData<S>,
    _error: PhantomData<E>,
}

pub struct Builder<S = ()> {
    value: i64,
    _state: PhantomData<S>,
}

impl<S> Builder<S> {
    pub fn new() -> Self {
        Self { value: 0, _state: PhantomData }
    }

    pub fn attach(self, lane: Lane<S>) -> Self {
        Self { value: lane.value, _state: PhantomData }
    }

    pub fn value(&self) -> i64 { self.value }
}

pub fn make_lane<H, S>(handler: H) -> Lane<S>
where
    H: FnOnce() -> i64,
{
    Lane { value: handler(), _state: PhantomData, _error: PhantomData }
}

pub struct DefaultOnly<T = std::convert::Infallible> {
    value: i64,
    _state: PhantomData<T>,
}

impl DefaultOnly {
    pub fn new() -> Self {
        Self { value: 11, _state: PhantomData }
    }

    pub fn value(&self) -> i64 { self.value }
}
