use std::fmt;

#[derive(Debug)]
pub struct ExternalError;

impl fmt::Display for ExternalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("external failure")
    }
}

impl std::error::Error for ExternalError {}
