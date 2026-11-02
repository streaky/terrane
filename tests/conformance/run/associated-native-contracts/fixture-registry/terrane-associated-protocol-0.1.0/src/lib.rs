use std::fmt;

#[derive(Debug)]
pub struct Failure;
impl fmt::Display for Failure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str("request rejected")
    }
}
impl std::error::Error for Failure {}

pub trait Request {
    type Response;
    fn execute(self) -> Result<Self::Response, Failure>;
}

#[derive(Clone, Debug)]
pub struct SharedReply {
    pub label: String,
}

pub trait Marker {}
impl<T: Marker> Request for T {
    type Response = bool;
    fn execute(self) -> Result<Self::Response, Failure> { Ok(true) }
}
