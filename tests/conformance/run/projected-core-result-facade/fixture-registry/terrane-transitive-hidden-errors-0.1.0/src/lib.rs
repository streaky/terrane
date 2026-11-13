pub type Result<T, E = OldHiddenFailure> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct OldHiddenFailure;

impl std::fmt::Display for OldHiddenFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("old hidden failure")
    }
}

impl std::error::Error for OldHiddenFailure {}
