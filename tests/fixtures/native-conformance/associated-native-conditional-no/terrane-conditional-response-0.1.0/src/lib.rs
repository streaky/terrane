pub trait Enabled {}
pub trait Request { type Response; fn response(self) -> Self::Response; }
impl<T: Enabled> Request for T {
    type Response = bool;
    fn response(self) -> Self::Response { true }
}
#[derive(Clone, Debug)]
pub struct Allowed;
impl Enabled for Allowed {}
#[derive(Clone, Debug)]
pub struct Denied;
pub fn allowed() -> Allowed { Allowed }
pub fn denied() -> Denied { Denied }
pub fn execute<T: Request>(request: T) -> T::Response { request.response() }
