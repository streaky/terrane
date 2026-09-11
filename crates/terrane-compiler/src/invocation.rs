use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InvocationMode {
    Shared,
    Mutable,
    Consuming,
}

impl InvocationMode {
    pub(crate) fn accepts(self, actual: Self) -> bool {
        actual <= self
    }

    pub(crate) fn source_prefix(self) -> &'static str {
        match self {
            Self::Shared => "",
            Self::Mutable => "mutable ",
            Self::Consuming => "consuming ",
        }
    }

    pub(crate) fn reflection_name(self) -> &'static str {
        match self {
            Self::Shared => "shared",
            Self::Mutable => "mutable",
            Self::Consuming => "consuming",
        }
    }
}
