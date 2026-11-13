pub type Result<T, E = NewFailure> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct NewFailure;

impl std::fmt::Display for NewFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("transitive hidden failure")
    }
}

impl std::error::Error for NewFailure {}
