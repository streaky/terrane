use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    Transport,
    Protocol,
}

impl fmt::Display for Fault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport => formatter.write_str("transport failed"),
            Self::Protocol => formatter.write_str("protocol failed"),
        }
    }
}

impl std::error::Error for Fault {}

pub struct Channel {
    mode: i64,
}


#[must_use]
pub fn channel(mode: i64) -> Channel {
    Channel { mode }
}
impl Channel {
    #[must_use]
    pub fn new(mode: i64) -> Self {
        Self { mode }
    }

    pub fn next(&mut self) -> Option<Result<String, Fault>> {
        match self.mode {
            0 => None,
            1 => Some(Ok("message".to_owned())),
            _ => Some(Err(Fault::Transport)),
        }
    }

    pub fn outer(&self) -> Result<Option<String>, Fault> {
        match self.mode {
            0 => Ok(None),
            1 => Ok(Some("value".to_owned())),
            _ => Err(Fault::Protocol),
        }
    }
}
