use std::collections::{BTreeMap, BTreeSet};

pub trait Sample {
    fn sample() -> Self;
}

impl Sample for Option<i64> {
    fn sample() -> Self {
        Some(7)
    }
}

impl Sample for Vec<u8> {
    fn sample() -> Self {
        b"abc".to_vec()
    }
}

impl Sample for Vec<i64> {
    fn sample() -> Self {
        vec![1, 2]
    }
}

impl Sample for BTreeMap<String, i64> {
    fn sample() -> Self {
        BTreeMap::from([("answer".to_owned(), 42)])
    }
}

impl Sample for BTreeSet<String> {
    fn sample() -> Self {
        BTreeSet::from(["present".to_owned()])
    }
}

pub fn default_value<T: Default>() -> T {
    T::default()
}

pub fn sample_value<T: Sample>() -> T {
    T::sample()
}
