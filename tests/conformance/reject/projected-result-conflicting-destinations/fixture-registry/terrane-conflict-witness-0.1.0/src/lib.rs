use std::collections::HashMap;
use std::hash::Hash;

pub fn same_mapping<T: Eq + Hash>() -> HashMap<T, T> {
    HashMap::new()
}
