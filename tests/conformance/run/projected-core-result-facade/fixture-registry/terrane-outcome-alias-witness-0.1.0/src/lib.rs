pub type Result<T, E = Failure> = core::result::Result<T, E>;
pub type Verdict<T> = Result<T>;
pub type Maybe<T> = core::option::Option<T>;
pub type OpaqueResult<T, E = OpaqueFailure> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Failure;

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("alias failed")
    }
}

impl std::error::Error for Failure {}


#[derive(Debug)]
pub struct OpaqueFailure;

impl std::fmt::Display for OpaqueFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("opaque alias failed")
    }
}

impl std::error::Error for OpaqueFailure {}

pub trait Index<Row> {
    fn index(&self, row: &Row) -> Result<i64>;
}