use super::prelude::*;

/// Normalized coverage for finite alternatives. The same primitive serves
/// source enums, optional presence, and other closed semantic families.
#[derive(Default)]
pub(super) struct MatchCoverage {
    covered: BTreeSet<String>,
    catchall: bool,
}

impl MatchCoverage {
    pub(super) fn insert(&mut self, alternative: impl Into<String>) -> bool {
        if self.catchall {
            return false;
        }
        self.covered.insert(alternative.into())
    }

    pub(super) fn insert_catchall(&mut self) -> bool {
        if self.catchall {
            return false;
        }
        self.catchall = true;
        true
    }

    pub(super) fn missing<'a>(
        &self,
        alternatives: impl IntoIterator<Item = &'a str>,
    ) -> Vec<&'a str> {
        if self.catchall {
            return Vec::new();
        }
        alternatives
            .into_iter()
            .filter(|alternative| !self.covered.contains(*alternative))
            .collect()
    }
}
