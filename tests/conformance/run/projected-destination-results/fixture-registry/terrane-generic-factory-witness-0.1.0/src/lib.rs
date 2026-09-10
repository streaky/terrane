extern crate self as terrane_generic_factory_witness;

use std::collections::{BTreeMap, BTreeSet};

pub trait Sample {
    fn sample() -> Self;
}
pub trait Decode<'value> {
    fn decode(value: &'value str) -> Self;
}

impl Decode<'_> for String {
    fn decode(value: &str) -> Self {
        value.to_owned()
    }
}


impl Sample for i64 {
    fn sample() -> Self {
        42
    }
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

pub fn sample_rows<T: Sample>() -> BTreeMap<String, T> {
    BTreeMap::from([("row".to_owned(), T::sample())])
}
pub fn sample_numeric_rows<T: Sample>() -> BTreeMap<i64, T> {
    BTreeMap::from([(1, T::sample())])
}


pub fn renamed_bound_value<T>() -> T
where
    for<'value> T: terrane_generic_factory_witness::Decode<'value>,
{
    T::decode("renamed bound")
}
